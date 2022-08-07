use std::{collections::HashMap, fmt::Debug};

use anyhow::{bail, Context, Result};

use crate::{
    ast::{
        CallMode, Declaration, Expr, Function, Literal, Module, OptionalMode, Source, Statement,
        Structs, Type,
    },
    compiler::specify_source_in_input,
    ir::{IrBlock, IrElement, IrTerm, IrType},
};

#[derive(Debug)]
struct IrFunctionGenerator<'s> {
    source_code: &'s str,
    ir: Vec<IrElement>,
    args: &'s HashMap<String, usize>,
    fresh_var_index: usize,
    structs: &'s Structs,
    strings: &'s mut Vec<String>,
}

impl<'s> IrFunctionGenerator<'s> {
    pub fn new(
        source_code: &'s str,
        args: &'s HashMap<String, usize>,
        structs: &'s Structs,
        strings: &'s mut Vec<String>,
    ) -> IrFunctionGenerator<'s> {
        IrFunctionGenerator {
            source_code,
            ir: vec![],
            args,
            fresh_var_index: 0,
            structs,
            strings,
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
            Expr::Var(v, _typ) => {
                assert!(v.len() <= 2);

                if v.len() == 1 {
                    let v = &v[0];
                    if self.args.contains_key(v) {
                        Ok(IrElement::Term(IrTerm::Argument(self.args[v])))
                    } else {
                        // special treatment for panic instruction
                        // FIXME: implement function meta attributes
                        if v == "_panic" {
                            let meta = self.expr(&Source::unknown(Expr::Call(
                                CallMode::Function,
                                Box::new(Source::unknown(Expr::Var(
                                    vec!["_println".to_string()],
                                    Type::Fn(
                                        vec![Type::Ref(Box::new(Type::Byte))],
                                        Box::new(Type::Nil),
                                    ),
                                ))),
                                vec![Source::unknown(Expr::Lit(
                                    Literal::String(specify_source_in_input(
                                        self.source_code,
                                        expr.start.unwrap(),
                                        expr.end.unwrap(),
                                    )),
                                    Type::Infer(0),
                                ))],
                            )))?;
                            self.ir.push(meta);
                        }

                        Ok(IrElement::Term(IrTerm::Ident(v.clone())))
                    }
                } else {
                    Ok(IrElement::Term(IrTerm::Ident(format!("{}_{}", v[0], v[1]))))
                }
            }
            Expr::Method(subj, v, _typ) => Ok(IrElement::Term(IrTerm::Ident(format!(
                "{}_{}",
                subj.method_selector_name()?,
                v
            )))),
            Expr::Lit(literal, _typ) => match literal {
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
            Expr::Call(CallMode::Function, f, args) => {
                // in: f(a,b,c)
                // out: (call f a b c)
                let mut elements = vec![];
                elements.push(self.expr(f.as_ref())?);

                for arg in args {
                    elements.push(self.expr(&arg)?);
                }

                Ok(IrElement::i_call_raw(elements))
            }
            Expr::Call(CallMode::Array(_), f, args) => {
                // array[T] = tuple[addr[T]]
                // arr(i)= arr->1->i
                let fresh = self.var_fresh();

                // make sure that array value is an address
                let f_value = self.expr(f.as_ref())?;
                self.ir.push(IrElement::i_let(fresh.clone(), f_value));

                Ok(IrElement::i_addr_index(
                    IrElement::i_offset(IrElement::ident(fresh), 0),
                    self.expr(&args[0])?,
                ))
            }
            Expr::Call(CallMode::SizedArray, f, args) => Ok(IrElement::i_index(
                self.expr(f.as_ref())?,
                self.expr(&args[0])?,
            )),
            Expr::Struct(struct_name, exprs) => {
                // in: A { a: 1, b: 2 }
                // out: (let (data TYPE 1 2))
                let mut data = vec![];

                // FIXME: field order
                for (label, expr, typ) in exprs {
                    let mut result = self.expr(expr)?;

                    // coerce check
                    let actual_size = self.ir_type(typ)?.size_of();
                    let expected_size = self
                        .ir_type(&self.structs.get_projection_type(&struct_name, label)?)?
                        .size_of();
                    if expected_size != actual_size {
                        assert!(actual_size <= expected_size);
                        assert!(expected_size >= 2);

                        result = IrElement::i_coerce(actual_size, expected_size, result);
                    }

                    data.push(result);
                }

                Ok(IrElement::i_tuple(
                    self.ir_type(&Type::Struct(struct_name.clone()))?,
                    data,
                ))
            }
            Expr::Project(method, _, _proj, _label) if *method => {
                unreachable!()
            }
            Expr::Project(_, proj_typ, proj, label) => {
                let struct_name = &proj_typ.method_selector_name()?;
                let index = self.structs.get_projection_offset(struct_name, label)?;
                let value = self.expr(proj)?;

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
                    IrElement::i_call("_new", vec![IrElement::i_size_of(self.ir_type(t)?)]),
                ));

                let e_value = self.expr(e)?;
                self.ir.push(IrElement::i_assign(
                    IrElement::i_deref(IrElement::Term(IrTerm::Ident(v.clone()))),
                    e_value,
                ));

                Ok(IrElement::Term(IrTerm::Ident(v)))
            }
            Expr::Deref(e, _) => Ok(IrElement::i_deref(self.expr(e)?)),
            Expr::As(e, current, expected) => {
                let current = self.ir_type(current)?;
                let expected = self.ir_type(expected)?;
                if current.size_of() != expected.size_of() {
                    unreachable!();
                }

                self.expr(e)
            }
            Expr::Address(e, _) => {
                // You cannot just take the address of an immidiate value, so declare as a variable
                let next = match e.data {
                    Expr::Lit(_, _) | Expr::Struct(_, _) | Expr::Call(_, _, _) => {
                        let v = self.var_fresh();
                        let value = self.expr(e)?;
                        self.ir.push(IrElement::i_let(v.clone(), value));

                        IrElement::ident(v)
                    }
                    _ => self.expr(e)?,
                };

                Ok(IrElement::i_address(next))
            }
            Expr::Make(t, args) => match t {
                Type::SizedArray(arr, len) => {
                    assert_eq!(args.len(), 1);
                    let value = self.expr(&args[0])?;
                    Ok(IrElement::i_slice(*len, self.ir_type(arr)?, value))
                }
                Type::Array(arr) => {
                    if args.len() != 2 {
                        bail!("array constructor takes 2 arguments, found {}", args.len());
                    }

                    let len = self.expr(&args[0])?;
                    let value = self.expr(&args[1])?;
                    Ok(IrElement::i_tuple(
                        self.ir_type(t)?,
                        vec![IrElement::i_slice_raw(len, self.ir_type(arr)?, value)],
                    ))
                }
                _ => unreachable!(),
            },
            Expr::Unwrap(expr, _) => Ok(IrElement::i_deref(self.expr(expr)?)),
            Expr::Optional(mode, _, expr) => {
                let value = self.expr(expr)?;
                let result = self.var_fresh();
                self.ir.push(IrElement::i_let(result.clone(), value));

                Ok(match mode {
                    OptionalMode::Nil => IrElement::nil(),
                    OptionalMode::Some => IrElement::i_address(IrElement::ident(result)),
                })
            }
        }
    }

    fn statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Let(x, e, _) => {
                let v = self.expr(e)?;
                self.ir.push(IrElement::i_let(x.to_string(), v));
            }
            Statement::Expr(e, t) => {
                let v = self.expr(e)?;
                self.ir.push(v);
                self.ir.push(IrElement::i_pop(self.ir_type(t)?));
            }
            Statement::Return(e, _) => {
                let v = self.expr(e)?;
                self.ir.push(IrElement::i_return(v));
            }
            Statement::If(b, s1, s2) => {
                let v = self.expr(b)?;
                let gen1 = {
                    let mut generator = IrFunctionGenerator::new(
                        self.source_code,
                        self.args,
                        &self.structs,
                        &mut self.strings,
                    );
                    generator.statements(&s1)?;
                    generator.ir
                };
                let gen2 = {
                    let mut generator = IrFunctionGenerator::new(
                        self.source_code,
                        self.args,
                        &self.structs,
                        &mut self.strings,
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
            Statement::Assignment(_typ, lhs, rhs) => {
                let lhs_value = self.expr(lhs)?;
                let rhs_value = self.expr(rhs)?;

                self.ir.push(IrElement::i_assign(lhs_value, rhs_value))
            }
            Statement::While(cond, body) => {
                let vcond = self.expr(cond)?;
                let gen = {
                    let mut generator = IrFunctionGenerator::new(
                        self.source_code,
                        self.args,
                        &self.structs,
                        &mut self.strings,
                    );
                    generator.statements(&body)?;
                    generator.ir
                };

                self.ir.push(IrElement::block(
                    "while",
                    vec![vcond, IrElement::block("seq", gen)],
                ));
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
    source_code: &'s str,
    structs: Structs,
    strings: Vec<String>,
}

impl<'s> IrGenerator<'s> {
    pub fn new(source_code: &'s str) -> IrGenerator<'s> {
        IrGenerator {
            source_code,
            structs: Structs(HashMap::new()),
            strings: vec![],
        }
    }

    pub fn set_source_code(&'s mut self, source_code: &'s str) {
        self.source_code = source_code;
    }

    pub fn context(&mut self, structs: Structs) {
        self.structs = structs;
    }

    pub fn ir_type(&self, typ: &Type) -> Result<IrType> {
        IrType::from_type_ast(typ, &self.structs)
    }

    pub fn function(&mut self, function: &Function) -> Result<IrElement> {
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

        let mut generator =
            IrFunctionGenerator::new(self.source_code, &args, &self.structs, &mut self.strings);

        generator.function(&function.body)?;

        Ok(IrElement::d_func(
            &function.name,
            arg_types_in_ir,
            Box::new(return_type),
            generator.ir,
        ))
    }

    pub fn method(&mut self, typ: &Type, function: &Function) -> Result<IrElement> {
        let mut args = HashMap::new();
        let mut arg_index = 0;
        let mut arg_types_in_ir = vec![];

        // argument in reverse order
        for (name, typ) in function.args.iter().rev() {
            arg_index += 1; // self.stack_size_of(typ)?;
            args.insert(name.clone(), arg_index - 1);
            arg_types_in_ir.push(
                self.ir_type(typ)
                    .context(format!("at method: {:?}::{}", typ, function.name))?,
            );
        }

        let return_type = self
            .ir_type(&function.return_type)
            .context(format!("[return type] {:?}", function.return_type))?;

        let mut generator =
            IrFunctionGenerator::new(self.source_code, &args, &self.structs, &mut self.strings);

        generator.function(&function.body)?;

        // FIXME: method block for ITable
        Ok(IrElement::d_func(
            format!("{}_{}", typ.method_selector_name()?, function.name),
            arg_types_in_ir,
            Box::new(return_type),
            generator.ir,
        ))
    }

    pub fn variable(
        &mut self,
        name: &String,
        expr: &Source<Expr>,
        typ: &Type,
    ) -> Result<IrElement> {
        let empty = HashMap::new();
        let typ = self.ir_type(typ)?;
        let mut generator =
            IrFunctionGenerator::new(self.source_code, &empty, &self.structs, &mut self.strings);

        Ok(IrElement::d_var(name, typ, generator.expr(expr)?))
    }

    pub fn module(&mut self, module: &Module) -> Result<IrElement> {
        let mut elements = vec![];

        for decl in &module.0 {
            match decl {
                Declaration::Function(f) => {
                    // skip if this function is not used
                    if f.dead_code {
                        continue;
                    }

                    elements.push(self.function(&f).context(format!("function {}", f.name))?);
                }
                Declaration::Method(typ, f) => {
                    // skip if this function is not used
                    if f.dead_code {
                        continue;
                    }

                    elements.push(self.method(typ, f).context(format!("method {}", f.name))?);
                }
                Declaration::Variable(v, expr, t) => {
                    elements.push(
                        self.variable(v, expr, t)
                            .context(format!("variable {}", v))?,
                    );
                }
                Declaration::Struct(_) => {}
            }
        }

        // string segment
        let mut strings = self
            .strings
            .iter()
            .map(|t| {
                let mut bytes = vec![IrElement::int(t.as_bytes().len() as i32)];
                bytes.extend(
                    t.as_bytes()
                        .iter()
                        .map(|i| IrElement::int(*i as i32))
                        .collect::<Vec<_>>(),
                );

                IrElement::block("text", bytes)
            })
            .collect::<Vec<_>>();
        strings.extend(elements);

        Ok(IrElement::Block(IrBlock {
            name: "module".to_string(),
            elements: strings,
        }))
    }

    pub fn generate(&mut self, module: &Module) -> Result<IrElement> {
        self.module(module)
    }
}

pub fn generate(source_code: &str, module: &Module) -> Result<IrElement> {
    let mut g = IrGenerator::new(source_code);
    let code = g.generate(module)?;

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
    (func $Point_sum (args (address (tuple $int $int))) (return $int)
        (return (call
            $_add
            (addr_offset $0 0)
            (addr_offset $0 1)
        ))
    )
    (func $main (args) (return $int)
        (let $p (tuple (tuple $int $int) 10 20))
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
    (text 3 102 111 111)
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
                    &format!("struct string {{ data: bytes }}\n{}", code),
                    "main".to_string(),
                )
                .unwrap();
            info!("{}", generated.show());

            let element = parse_ir(ir_code).unwrap();

            assert_eq!(generated, element, "{}", code);
        }
    }
}
