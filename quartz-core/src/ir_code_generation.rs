use std::{collections::HashMap, fmt::Debug};

use anyhow::{Context, Result};

use crate::{
    ast::{
        CallMode, Declaration, Expr, Function, Literal, Module, OptionalMode, Source, Statement,
        Struct, Structs, Type,
    },
    compiler::SourceLoader,
    ir::{IrBlock, IrElement, IrTerm, IrType},
};

#[derive(Debug)]
struct IrFunctionGenerator<'s> {
    ir: Vec<IrElement>,
    args: &'s HashMap<String, usize>,
    fresh_var_index: usize,
    structs: &'s Structs,
    strings: &'s mut Vec<String>,
    source_loader: &'s SourceLoader,
    module_path: &'s str,
}

impl<'s> IrFunctionGenerator<'s> {
    pub fn new(
        source_loader: &'s SourceLoader,
        args: &'s HashMap<String, usize>,
        structs: &'s Structs,
        strings: &'s mut Vec<String>,
        module_path: &'s str,
    ) -> IrFunctionGenerator<'s> {
        IrFunctionGenerator {
            source_loader,
            ir: vec![],
            args,
            fresh_var_index: 0,
            structs,
            strings,
            module_path,
        }
    }

    pub fn ir_type(&self, typ: &Type) -> Result<IrType> {
        IrType::from_type_ast(typ, self.structs)
    }

    pub fn var_fresh(&mut self) -> String {
        self.fresh_var_index += 1;

        format!("fresh_{}", self.fresh_var_index)
    }

    pub fn expr(&mut self, expr: &Source<Expr>) -> Result<IrElement> {
        match &expr.data {
            Expr::Var(v) => {
                assert!(v.len() <= 2);

                if v.len() == 1 {
                    let v = &v[0];
                    if self.args.contains_key(v) {
                        Ok(IrElement::Term(IrTerm::Argument(self.args[v])))
                    } else {
                        // special treatment for panic instruction
                        // FIXME: implement function meta attributes
                        if v == "_panic" {
                            let meta = self.expr(&Source::unknown(Expr::function_call(
                                Source::unknown(Expr::Var(vec!["_println".to_string()])),
                                vec![Source::unknown(Expr::Lit(Literal::String(
                                    self.source_loader.specify_source(
                                        self.module_path,
                                        expr.start.unwrap(),
                                        expr.end.unwrap(),
                                    )?,
                                )))],
                            )))?;
                            self.ir.push(meta);
                        }

                        Ok(IrElement::Term(IrTerm::Ident(v.clone())))
                    }
                } else {
                    Ok(IrElement::Term(IrTerm::Ident(format!("{}_{}", v[0], v[1]))))
                }
            }
            Expr::PathVar(subj, v) => Ok(IrElement::Term(IrTerm::Ident(format!(
                "{}_{}",
                subj.method_selector_name()
                    .context(format!("{:?}::{}", subj, v))?,
                v
            )))),
            Expr::Lit(literal) => match literal {
                Literal::Nil => Ok(IrElement::Term(IrTerm::Nil)),
                Literal::Bool(b) => Ok(IrElement::Term(IrTerm::Bool(*b))),
                Literal::Int(n) => Ok(IrElement::Term(IrTerm::Int(*n))),
                Literal::String(s) => {
                    let t = self.strings.len();
                    self.strings.push(s.clone());

                    Ok(IrElement::block("string", vec![IrElement::int(t as i32)]))
                }
                Literal::Array(arr, _) => {
                    let v = self.var_fresh();
                    let n = arr.len() as i32;

                    // in: [1,2]
                    //
                    // out: (let $v (new 2))
                    //      (assign (offset $v 0) 1)
                    //      (assign (offset $v 1) 2)
                    //      $v
                    self.ir.push(IrElement::i_let(
                        v.clone(),
                        IrElement::i_call("_new", vec![IrElement::int(n)]),
                    ));

                    for (i, elem) in arr.into_iter().enumerate() {
                        let velem = self.expr(&elem)?;

                        self.ir.push(IrElement::i_assign(
                            IrElement::i_offset(IrElement::Term(IrTerm::Ident(v.clone())), i),
                            velem,
                        ));
                    }

                    todo!()
                    // Ok(IrElement::Term(IrTerm::Ident(v, size)))
                }
            },
            Expr::Call(CallMode::Array, f, _, args) => Ok(IrElement::i_addr_index(
                self.expr(f.as_ref())?,
                self.expr(&args[0])?,
            )),
            Expr::Call(CallMode::SizedArray, f, _, args) => Ok(IrElement::i_index(
                self.expr(f.as_ref())?,
                self.expr(&args[0])?,
            )),
            Expr::Call(CallMode::Function, f, types, args) => {
                println!("{:?} {:?} {:?}", f, types, args);
                // in: f(a,b,c)
                // out: (call f a b c)
                let mut elements = vec![];
                elements.push(self.expr(f.as_ref())?);

                for t in types {
                    elements.push(self.ir_type(t)?.to_element());
                }

                for arg in args {
                    elements.push(self.expr(&arg)?);
                }

                Ok(IrElement::i_call_raw(elements))
            }
            Expr::AssociatedCall(CallMode::Function, type_, label, args) => {
                let mut elements = vec![];
                elements.push(IrElement::Term(IrTerm::Ident(format!(
                    "{}_{}",
                    type_
                        .method_selector_name()
                        .context(format!("{:?}::{}", type_, label))?,
                    label
                ))));

                for app in type_.type_applications()? {
                    elements.push(self.ir_type(&app)?.to_element());
                }

                for arg in args {
                    elements.push(self.expr(&arg)?);
                }

                Ok(IrElement::i_call_raw(elements))
            }
            Expr::AssociatedCall(_, _, _, _) => unreachable!(),
            Expr::MethodCall(CallMode::Array, type_, label, self_, args) => {
                Ok(IrElement::i_addr_index(
                    self.expr(&Source::unknown(Expr::Project(
                        type_.clone(),
                        self_.clone(),
                        label.clone(),
                    )))?,
                    self.expr(&args[0])?,
                ))
            }
            Expr::MethodCall(CallMode::SizedArray, _, _, _, _) => unreachable!(),
            Expr::MethodCall(CallMode::Function, type_, label, self_, args) => {
                let mut elements = vec![];
                elements.push(self.expr(&Source::unknown(Expr::Var(vec![
                    type_.method_selector_name().context(format!(
                        "method selector name for {:?}::{}, {}",
                        type_,
                        label,
                        self.source_loader.specify_source(
                            self.module_path,
                            expr.start.unwrap(),
                            expr.end.unwrap(),
                        )?,
                    ))?,
                    label.clone(),
                ])))?);

                elements.push(self.expr(self_)?);
                for arg in args {
                    elements.push(self.expr(&arg)?);
                }

                Ok(IrElement::i_call_raw(elements))
            }
            Expr::Struct(struct_name, params, exprs) => {
                // in: A { a: 1, b: 2 }
                // out: (let (data TYPE 1 2))
                let mut data = vec![];

                // FIXME: field order
                for (_, expr, _) in exprs {
                    let result = self.expr(expr)?;
                    data.push(result);
                }

                Ok(IrElement::i_tuple(
                    self.ir_type(&Type::type_app_or(
                        Type::Struct(struct_name.clone()),
                        params.clone(),
                    ))?,
                    data,
                ))
            }
            Expr::Project(proj_typ, proj, label) => {
                let struct_name = &proj_typ
                    .method_selector_name()
                    .context(format!("{:?}.{}", proj.data, label))?;
                let index = self.structs.get_projection_offset(struct_name, label)?;
                // `f().x` should be compiled to `((let $v (call $f)) (offset $v 0))`
                let value = {
                    let result = self.expr(proj)?;

                    if result.is_address_expr() {
                        result
                    } else {
                        let v = self.var_fresh();
                        self.ir.push(IrElement::i_let(v.clone(), result));

                        IrElement::ident(v)
                    }
                };

                Ok(if let Some(_t) = proj_typ.as_ref_type() {
                    // if p is a pointer in p.l, it should be compiled to p->l
                    IrElement::i_addr_offset(value, index)
                } else {
                    IrElement::i_offset(value, index)
                })
            }
            Expr::Ref(e, t) => {
                let v = self.var_fresh();
                self.ir.push(IrElement::i_let(
                    v.clone(),
                    IrElement::i_alloc(self.ir_type(t)?, IrElement::int(1)),
                ));

                let e_value = self.expr(e)?;
                self.ir.push(IrElement::i_assign(
                    IrElement::i_addr_index(IrElement::ident(v.clone()), IrElement::int(0)),
                    e_value,
                ));

                Ok(IrElement::i_address(IrElement::i_addr_index(
                    IrElement::ident(v),
                    IrElement::int(0),
                )))
            }
            Expr::Deref(e, _) => Ok(IrElement::i_deref(self.expr(e)?)),
            Expr::As(e, _, expected) => {
                Ok(IrElement::i_coerce(self.expr(e)?, self.ir_type(expected)?))
            }
            Expr::Address(e) => {
                let value = {
                    let result = self.expr(e)?;

                    if result.is_address_expr() {
                        result
                    } else {
                        let v = self.var_fresh();
                        self.ir.push(IrElement::i_let(v.clone(), result));

                        IrElement::ident(v)
                    }
                };

                Ok(IrElement::i_address(value))
            }
            Expr::Make(t, args) => match t {
                Type::SizedArray(arr, len) => {
                    assert_eq!(args.len(), 1);
                    let value = self.expr(&args[0])?;
                    Ok(IrElement::i_slice(*len, self.ir_type(arr)?, value))
                }
                Type::Array(arr) if args.len() == 1 => {
                    let len = self.expr(&args[0])?;
                    let len_var = self.var_fresh();
                    self.ir.push(IrElement::i_let(len_var.clone(), len));

                    let array = self.var_fresh();
                    self.ir.push(IrElement::i_let(
                        array.clone(),
                        IrElement::i_alloc(self.ir_type(arr)?, IrElement::ident(len_var.clone())),
                    ));

                    Ok(IrElement::ident(array))
                }
                Type::Array(arr) if args.len() == 2 => {
                    let len = self.expr(&args[0])?;
                    let value = self.expr(&args[1])?;

                    /*
                        (let $array (alloc $type $len))
                        (let $i 0)
                        (while (_lt $i $len)
                            (assign $array->$i $value)
                            (assign $i (_add $i 1))
                        )
                    */
                    let len_var = self.var_fresh();
                    self.ir.push(IrElement::i_let(len_var.clone(), len));

                    let array = self.var_fresh();
                    self.ir.push(IrElement::i_let(
                        array.clone(),
                        IrElement::i_alloc(self.ir_type(arr)?, IrElement::ident(len_var.clone())),
                    ));

                    let i = self.var_fresh();
                    self.ir.push(IrElement::i_let(i.clone(), IrElement::int(0)));
                    self.ir.push(IrElement::i_while(
                        IrElement::i_call(
                            "_lt",
                            vec![
                                IrElement::Term(IrTerm::Ident(i.clone())),
                                IrElement::ident(len_var),
                            ],
                        ),
                        vec![
                            IrElement::i_assign(
                                IrElement::i_addr_index(
                                    IrElement::Term(IrTerm::Ident(array.clone())),
                                    IrElement::Term(IrTerm::Ident(i.clone())),
                                ),
                                value,
                            ),
                            IrElement::i_assign(
                                IrElement::Term(IrTerm::Ident(i.clone())),
                                IrElement::i_call(
                                    "_add",
                                    vec![
                                        IrElement::Term(IrTerm::Ident(i.clone())),
                                        IrElement::int(1),
                                    ],
                                ),
                            ),
                        ],
                    ));

                    Ok(IrElement::ident(array))
                }
                _ => unreachable!(),
            },
            Expr::Unwrap(expr, _) => Ok(IrElement::i_deref(self.expr(expr)?)),
            Expr::Optional(mode, typ, expr) => {
                let value = self.expr(expr)?;
                let result = self.var_fresh();
                self.ir.push(IrElement::i_let(result.clone(), value));

                Ok(match mode {
                    OptionalMode::Nil => IrElement::nil(),
                    OptionalMode::Some => self.expr(&Source::unknown(Expr::Ref(
                        Box::new(Source::unknown(Expr::Var(vec![result]))),
                        typ.as_optional().unwrap().as_ref().clone(),
                    )))?,
                })
            }
        }
    }

    fn statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Let(x, e) => {
                let v = self.expr(e)?;
                self.ir.push(IrElement::i_let(x.to_string(), v));
            }
            Statement::Expr(e, t) => {
                let v = self.expr(e)?;
                self.ir.push(v);
                self.ir.push(IrElement::i_pop(self.ir_type(t)?));
            }
            Statement::Return(e) => {
                let v = self.expr(e)?;
                self.ir.push(IrElement::i_return(v));
            }
            Statement::If(b, s1, s2) => {
                let v = self.expr(b)?;
                let gen1 = {
                    let mut generator = IrFunctionGenerator::new(
                        self.source_loader,
                        self.args,
                        &self.structs,
                        &mut self.strings,
                        self.module_path,
                    );
                    generator.statements(&s1)?;
                    generator.ir
                };
                let gen2 = {
                    let mut generator = IrFunctionGenerator::new(
                        self.source_loader,
                        self.args,
                        &self.structs,
                        &mut self.strings,
                        self.module_path,
                    );
                    generator.statements(&s2)?;
                    generator.ir
                };

                self.ir.push(IrElement::block(
                    "if",
                    vec![
                        v,
                        IrElement::block("seq", gen1),
                        IrElement::block("seq", gen2),
                    ],
                ));
            }
            Statement::Continue => self.ir.push(IrElement::block("continue", vec![])),
            Statement::Assignment(lhs, rhs) => {
                let lhs_value = self.expr(lhs)?;
                let rhs_value = self.expr(rhs)?;

                self.ir.push(IrElement::i_assign(lhs_value, rhs_value))
            }
            Statement::While(cond, body) => {
                let vcond = self.expr(cond)?;
                let gen = {
                    let mut generator = IrFunctionGenerator::new(
                        self.source_loader,
                        self.args,
                        &self.structs,
                        &mut self.strings,
                        self.module_path,
                    );
                    generator.statements(&body)?;
                    generator.ir
                };

                self.ir.push(IrElement::i_while(vcond, gen));
            }
        }

        Ok(())
    }

    // for normal blocks
    pub fn statements(&mut self, statements: &Vec<Source<Statement>>) -> Result<()> {
        self.ir.push(IrElement::block("begin_scope", vec![]));

        for statement in statements {
            self.statement(&statement.data)?;
        }

        self.ir.push(IrElement::block("end_scope", vec![]));

        Ok(())
    }

    // for functions
    pub fn function(&mut self, statements: &Vec<Source<Statement>>) -> Result<()> {
        for statement in statements {
            self.statement(&statement.data)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct IrGenerator<'s> {
    source_loader: &'s SourceLoader,
    structs: Structs,
    strings: Vec<String>,
    module_path: String,
}

impl<'s> IrGenerator<'s> {
    pub fn new(source_loader: &'s SourceLoader) -> IrGenerator<'s> {
        IrGenerator {
            source_loader,
            structs: Structs(HashMap::new()),
            strings: vec![],
            module_path: String::new(),
        }
    }

    pub fn set_source_loader(&'s mut self, source_loader: &'s SourceLoader) {
        self.source_loader = source_loader;
    }

    pub fn context(&mut self, structs: Structs) {
        self.structs = structs;
    }

    fn ir_type(&self, typ: &Type) -> Result<IrType> {
        IrType::from_type_ast(typ, &self.structs)
    }

    fn function(&mut self, function: &Function) -> Result<IrElement> {
        let mut args = HashMap::new();
        let mut arg_index = 0;
        let mut arg_types_in_ir = vec![];

        // argument in reverse order
        for (name, typ) in function.args.iter().rev() {
            arg_index += 1; // self.stack_size_of(typ)?;
            args.insert(name.clone(), arg_index - 1);
            arg_types_in_ir.push(self.ir_type(typ)?);
        }

        let return_type = self.ir_type(&function.return_type)?;

        let mut generator = IrFunctionGenerator::new(
            self.source_loader,
            &args,
            &self.structs,
            &mut self.strings,
            &self.module_path,
        );

        generator.function(&function.body)?;

        Ok(IrElement::d_func(
            &function.name.data,
            arg_types_in_ir,
            Box::new(return_type),
            generator.ir,
        ))
    }

    fn method(&mut self, typ: &String, function: &Function) -> Result<IrElement> {
        let mut args = HashMap::new();
        let mut arg_index = 0;
        let mut arg_types_in_ir = vec![];

        // argument in reverse order
        for (name, typ) in function.args.iter().rev() {
            arg_index += 1; // self.stack_size_of(typ)?;
            args.insert(name.clone(), arg_index - 1);
            arg_types_in_ir.push(
                self.ir_type(typ)
                    .context(format!("at method: {:?}::{}", typ, function.name.data))?,
            );
        }

        let return_type = self
            .ir_type(&function.return_type)
            .context(format!("[return type] {:?}", function.return_type))?;

        let mut generator = IrFunctionGenerator::new(
            self.source_loader,
            &args,
            &self.structs,
            &mut self.strings,
            &self.module_path,
        );

        generator.function(&function.body)?;

        // FIXME: method block for ITable
        Ok(IrElement::d_func(
            format!("{}_{}", typ, function.name.data),
            arg_types_in_ir,
            Box::new(return_type),
            generator.ir,
        ))
    }

    fn variable(&mut self, name: &String, expr: &Source<Expr>, typ: &Type) -> Result<IrElement> {
        let empty = HashMap::new();
        let typ = self.ir_type(typ)?;
        let mut generator = IrFunctionGenerator::new(
            self.source_loader,
            &empty,
            &self.structs,
            &mut self.strings,
            &self.module_path,
        );

        Ok(IrElement::d_var(name, typ, generator.expr(expr)?))
    }

    fn struct_(&mut self, s: &Struct) -> Result<IrElement> {
        let tuple = IrType::tuple(
            s.fields
                .iter()
                .map(|(_, typ)| -> Result<IrType> {
                    self.ir_type(typ).context(format!("{:?}", typ))
                })
                .collect::<Result<_>>()?,
        );

        Ok(IrElement::d_type(
            s.name.clone(),
            if s.type_params.is_empty() {
                tuple
            } else {
                IrType::generic(tuple, s.type_params.clone())
            },
        ))
    }

    fn module(&mut self, module: &Module) -> Result<Vec<IrElement>> {
        self.module_path = module.module_path.clone();

        let mut elements = vec![];
        for decl in &module.decls {
            match decl {
                Declaration::Function(f) => {
                    // skip if this function is not used
                    if f.dead_code {
                        continue;
                    }

                    elements.push(
                        self.function(&f)
                            .context(format!("function {}", f.name.data))?,
                    );
                }
                Declaration::Method(typ, _, f) => {
                    // skip if this function is not used
                    if f.dead_code {
                        continue;
                    }

                    elements.push(
                        self.method(&typ.data, f)
                            .context(format!("method {}", f.name.data))?,
                    );
                }
                Declaration::Variable(v, expr, t) => {
                    elements.push(
                        self.variable(v, expr, t)
                            .context(format!("variable {}", v))?,
                    );
                }
                Declaration::Struct(s) => {
                    if s.dead_code {
                        continue;
                    }

                    elements.push(self.struct_(&s)?);
                }
                Declaration::Import(_) => {}
            }
        }

        Ok(elements)
    }

    pub fn generate_module(&mut self, module: &Module) -> Result<IrElement> {
        let mut elements = self.string_segment();
        elements.extend(self.module(&module)?);

        Ok(IrElement::block("module", elements))
    }

    fn string_segment(&mut self) -> Vec<IrElement> {
        self.strings
            .iter()
            .map(|t| {
                let mut bytes = vec![IrElement::int(t.as_bytes().len() as i32 + 1)];
                bytes.extend(
                    t.as_bytes()
                        .iter()
                        .map(|i| IrElement::int(*i as i32))
                        .collect::<Vec<_>>(),
                );

                IrElement::block("text", bytes)
            })
            .collect::<Vec<_>>()
    }

    pub fn generate(&mut self, modules: &Vec<Module>) -> Result<IrElement> {
        let mut elements = vec![];

        // require link
        for m in modules {
            elements.extend(self.module(m)?);
        }

        let mut result = self.string_segment();
        result.extend(elements);

        Ok(IrElement::Block(IrBlock {
            name: "module".to_string(),
            elements: result,
        }))
    }
}

pub fn generate(source_loader: &SourceLoader, module: &Module) -> Result<IrElement> {
    let mut g = IrGenerator::new(source_loader);
    let code = g.generate_module(module)?;

    Ok(code)
}

#[cfg(test)]
mod tests {
    use log::info;
    use pretty_assertions::assert_eq;

    use crate::{compiler::Compiler, ir::parse_ir};

    #[test]
    fn test_generate() {
        let cases = vec![
            (
                r#"
func main() {
    let x = 10;
    x = 20;
    return x;
}
"#,
                r#"
(module
    (func $main (args) (return $int)
        (let $x 10)
        (assign $x 20)
        (return $x)
    )
)
"#,
            ),
            (
                r#"
func f(c: int): int {
    let x = make[array[int,4]](3);
    x(0) = 1;
    x(1) = 2;
    x(2) = 3;
    x(2) = 4;

    return _add(_add(x(1), x(2)), c);
}

func main() {
    return f(10);
}
"#,
                r#"
(module
    (func $f (args $int) (return $int)
        (let $x (slice 4 $int 3))
        (assign (index $x 0) 1)
        (assign (index $x 1) 2)
        (assign (index $x 2) 3)

        (assign (index $x 2) 4)

        (return
            (call $_add
                (call $_add
                    (index $x 1)
                    (index $x 2))
                $0
            )
        )
    )
    (func $main (args) (return $int)
        (return (call $f 10))
    )
)
"#,
            ),
            (
                r#"
struct Point {
    x: int,
    y: int,
}

method Point sum(self): int {
    return _add(self.x, self.y);
}

func main() {
    let p = Point { x: 10, y: 20 };

    return p.sum();
}
"#,
                r#"
(module
    (type $Point (tuple $int $int))
    (func $Point_sum (args (address $Point)) (return $int)
        (return (call
            $_add
            (addr_offset $0 0)
            (addr_offset $0 1)
        ))
    )
    (func $main (args) (return $int)
        (let $p (tuple $Point 10 20))
        (return (call $Point_sum (address $p)))
    )
)
"#,
            ),
            // skip to bundle unused function
            (
                r#"
func f(c: int): int {
    return 10;
}

func main() {
    return nil;
}
"#,
                r#"
(module
    (func $main (args) (return $nil)
        (return nil)
    )
)
"#,
            ),
            // argument order
            (
                r#"
func f(c: int, b: bool): int {
    return c;
}

func main() {
    return f(0, true);
}
"#,
                r#"
(module
    (func $f (args $int $bool) (return $int)
        (return $1))
    (func $main (args) (return $int)
        (return (call $f 0 true))
    )
)
"#,
            ),
            (
                r#"
func main() {
    let s = "foo";

    return _println(s);
}
"#,
                r#"
(module
    (text 4 102 111 111)
    (func $main (args) (return $nil)
        (let $s (string 0))
        (return (call $_println $s))
    )
)
"#,
            ),
        ];

        for (code, ir_code) in cases {
            let mut compiler = Compiler::new();
            let generated = compiler
                .compile_ir_nostd(
                    // FIXME: define string for now
                    format!("struct string {{ data: bytes }}\n{}", code),
                    "main".to_string(),
                    None,
                )
                .unwrap();
            info!("{}", generated.show());

            let element = parse_ir(ir_code).unwrap();

            assert_eq!(generated, element, "{}", code);
        }
    }
}
