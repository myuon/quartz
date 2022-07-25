use std::{collections::HashMap, fmt::Debug};

use anyhow::{Context, Result};

use crate::{
    ast::{
        size_of, CallMode, Declaration, Expr, Function, Literal, Module, Source, Statement,
        Structs, Type,
    },
    compiler::specify_source_in_input,
    ir::{IrBlock, IrElement, IrTerm},
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

    pub fn var_fresh(&mut self) -> String {
        self.fresh_var_index += 1;

        format!("fresh_{}", self.fresh_var_index)
    }

    pub fn expr(&mut self, expr: &Source<Expr>) -> Result<IrElement> {
        match &expr.data {
            Expr::Var(v, typ) => {
                assert!(v.len() <= 2);

                if v.len() == 1 {
                    let v = &v[0];
                    if self.args.contains_key(v) {
                        Ok(IrElement::Term(IrTerm::Argument(
                            self.args[v],
                            size_of(typ, self.structs),
                        )))
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

                        Ok(IrElement::Term(IrTerm::Ident(
                            v.clone(),
                            size_of(typ, self.structs),
                        )))
                    }
                } else {
                    Ok(IrElement::Term(IrTerm::Ident(
                        format!("{}_{}", v[0], v[1]),
                        size_of(typ, self.structs),
                    )))
                }
            }
            Expr::Method(subj, v, typ) => Ok(IrElement::Term(IrTerm::Ident(
                format!("{}_{}", subj.method_selector_name()?, v),
                size_of(typ, self.structs),
            ))),
            Expr::Lit(literal, typ) => match literal {
                Literal::Nil => {
                    let s = size_of(typ, &self.structs);
                    Ok(if s > 1 {
                        IrElement::i_coerce(1, s, IrElement::Term(IrTerm::Nil))
                    } else {
                        IrElement::Term(IrTerm::Nil)
                    })
                }
                Literal::Bool(b) => Ok(IrElement::Term(IrTerm::Bool(*b))),
                Literal::Int(n) => Ok(IrElement::Term(IrTerm::Int(*n))),
                Literal::String(s) => {
                    let t = self.strings.len();
                    self.strings.push(s.clone());

                    Ok(IrElement::block(
                        "data",
                        vec![
                            IrElement::int(2),
                            IrElement::block("string", vec![IrElement::int(t as i32)]),
                        ],
                    ))
                }
                Literal::Array(arr, t) => {
                    let size = size_of(&Type::Array(Box::new(t.clone())), self.structs);
                    let v = self.var_fresh();
                    let n = arr.len() as i32;
                    let element_size = size_of(t, self.structs);

                    // in: [1,2]
                    //
                    // out: (let $v (new 2))
                    //      (assign (offset $v 0) 1)
                    //      (assign (offset $v 1) 2)
                    //      $v
                    self.ir.push(IrElement::i_let(
                        IrElement::ir_type(t, self.structs)?,
                        v.clone(),
                        IrElement::i_call("_new", vec![IrElement::int(n)]),
                    ));

                    for (i, elem) in arr.into_iter().enumerate() {
                        let velem = self.expr(&elem)?;

                        self.ir.push(IrElement::i_assign(
                            element_size,
                            IrElement::i_offset_im(
                                element_size,
                                IrElement::Term(IrTerm::Ident(v.clone(), size)),
                                i,
                            ),
                            velem,
                        ));
                    }

                    Ok(IrElement::Term(IrTerm::Ident(v, size)))
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
            Expr::Call(CallMode::Array, f, args) => {
                Ok(IrElement::i_offset(
                    1, // FIXME: correct value
                    self.expr(f.as_ref())?,
                    self.expr(&args[0])?,
                ))
            }
            Expr::Struct(struct_name, exprs) => {
                // in: A { a: 1, b: 2 }
                // out: (let (data POINTER_TO_INFO_TABLE 1 2))
                let mut data = vec![];
                // FIXME: POINTER_TO_INFO_TABLE
                data.push(IrElement::Term(IrTerm::Int(size_of(
                    &Type::Struct(struct_name.clone()),
                    &self.structs,
                ) as i32)));

                // FIXME: field order
                for (label, expr, typ) in exprs {
                    let mut result = self.expr(expr)?;

                    // coerce check
                    let actual_size = size_of(typ, self.structs);
                    let expected_size = size_of(
                        &self.structs.get_projection_type(&struct_name, label)?,
                        self.structs,
                    );
                    if expected_size != actual_size {
                        assert!(actual_size <= expected_size);
                        assert!(expected_size >= 2);

                        result = IrElement::i_coerce(actual_size, expected_size, result);
                    }

                    data.push(result);
                }

                Ok(IrElement::block("data", data))
            }
            Expr::Project(method, _struct_name, _proj, _label) if *method => {
                unreachable!()
            }
            Expr::Project(_, struct_name, proj, label) => {
                let index = self.structs.get_projection_offset(struct_name, label)?;
                let typ = self.structs.get_projection_type(struct_name, label)?;
                let value = self.expr(proj)?;

                Ok(IrElement::i_offset_im(
                    size_of(&typ, self.structs),
                    value,
                    index,
                ))
            }
            Expr::Index(arr, i) => Ok(IrElement::i_offset(
                1, // FIXME: correct value
                self.expr(arr.as_ref())?,
                self.expr(i.as_ref())?,
            )),
            Expr::Ref(e, t) => {
                let v = self.var_fresh();
                let size = size_of(t, self.structs);
                self.ir.push(IrElement::i_let(
                    IrElement::ir_type(&Type::Ref(Box::new(t.clone())), self.structs)?,
                    v.clone(),
                    IrElement::i_call("_new", vec![IrElement::int(size as i32)]),
                ));

                let e_value = self.expr(e)?;
                self.ir.push(IrElement::i_assign(
                    size,
                    IrElement::i_offset_im(
                        1,
                        IrElement::i_deref(1, IrElement::Term(IrTerm::Ident(v.clone(), 1))),
                        0,
                    ),
                    e_value,
                ));

                Ok(IrElement::Term(IrTerm::Ident(v, 1)))
            }
            Expr::Deref(e, t) => Ok(IrElement::i_deref(size_of(t, self.structs), self.expr(e)?)),
            Expr::As(e, current, expected) => {
                let current_size = size_of(current, self.structs);
                let expected_size = size_of(expected, self.structs);

                let value = self.expr(e)?;
                Ok(if current_size < expected_size {
                    IrElement::i_coerce(current_size, expected_size, value)
                } else {
                    value
                })
            }
            Expr::Address(e, _t) => Ok(IrElement::i_address(self.expr(e)?)),
            Expr::New(t, args) => match t {
                Type::SizedArray(arr, len) => {
                    let value = self.expr(&args[0])?;
                    let mut data = vec![IrElement::int((len * size_of(&arr, self.structs)) as i32)];
                    data.extend(std::iter::repeat(value).take(*len));

                    Ok(IrElement::block("data", data))
                }
                _ => unreachable!(),
            },
        }
    }

    fn statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Let(x, e, t) => {
                let v = self.expr(e)?;
                self.ir.push(IrElement::i_let(
                    IrElement::ir_type(t, self.structs)?,
                    x.to_string(),
                    v,
                ));
            }
            Statement::Expr(e, t) => {
                let v = self.expr(e)?;
                self.ir.push(v);
                self.ir.push(IrElement::instruction(
                    "pop",
                    vec![IrTerm::Int(size_of(t, self.structs) as i32)],
                ));
            }
            Statement::Return(e, t) => {
                let v = self.expr(e)?;
                self.ir.push(IrElement::block(
                    "return",
                    vec![
                        IrElement::Term(IrTerm::Int(size_of(t, self.structs) as i32)),
                        v,
                    ],
                ));
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
            Statement::Assignment(typ, lhs, rhs) => {
                let lhs_value = self.expr(lhs)?;
                let rhs_value = self.expr(rhs)?;

                self.ir.push(IrElement::i_assign(
                    size_of(typ, self.structs),
                    lhs_value,
                    rhs_value,
                ))
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
        // if last statement was not a return statement, insert it
        // FIXME: typecheck
        let need_return_nil_insert = !statements
            .last()
            .map(|s| matches!(s.data, Statement::Return(_, _)))
            .unwrap_or(false);

        for statement in statements {
            self.statement(&statement.data)?;
        }

        if need_return_nil_insert {
            self.statement(&Statement::Return(
                Source::unknown(Expr::Lit(Literal::Nil, Type::Nil)),
                Type::Nil,
            ))?;
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

    pub fn function(&mut self, function: &Function) -> Result<IrElement> {
        let mut elements = vec![IrElement::Term(IrTerm::Ident(function.name.clone(), 1))];
        let mut args = HashMap::new();
        let mut arg_index = 0;
        let mut arg_types_in_ir = vec![];

        // argument in reverse order
        for (name, typ) in function.args.iter().rev() {
            arg_index += 1; // self.stack_size_of(typ)?;
            args.insert(name.clone(), arg_index - 1);
            arg_types_in_ir.push(IrElement::ir_type(typ, &self.structs)?);
        }

        elements.push(IrElement::block(
            "args",
            arg_types_in_ir.into_iter().rev().collect(),
        ));

        let mut generator =
            IrFunctionGenerator::new(self.source_code, &args, &self.structs, &mut self.strings);

        generator.function(&function.body)?;

        elements.extend(generator.ir);

        Ok(IrElement::Block(IrBlock {
            name: "func".to_string(),
            elements,
        }))
    }

    pub fn method(&mut self, typ: &Type, function: &Function) -> Result<IrElement> {
        let mut elements = vec![IrElement::Term(IrTerm::Ident(
            format!("{}_{}", typ.method_selector_name()?, function.name),
            1,
        ))];
        let mut args = HashMap::new();
        let mut arg_index = 0;
        let mut arg_types_in_ir = vec![];

        // argument in reverse order
        for (name, typ) in function.args.iter().rev() {
            arg_index += 1; // self.stack_size_of(typ)?;
            args.insert(name.clone(), arg_index - 1);
            arg_types_in_ir.push(
                IrElement::ir_type(typ, &self.structs)
                    .context(format!("at method: {:?}::{}", typ, function.name))?,
            );
        }

        elements.push(IrElement::block(
            "args",
            arg_types_in_ir.into_iter().rev().collect(),
        ));

        let mut generator =
            IrFunctionGenerator::new(self.source_code, &args, &self.structs, &mut self.strings);

        generator.function(&function.body)?;

        elements.extend(generator.ir);

        // FIXME: method block for ITable
        Ok(IrElement::Block(IrBlock {
            name: "func".to_string(),
            elements,
        }))
    }

    pub fn variable(
        &mut self,
        name: &String,
        expr: &Source<Expr>,
        typ: &Type,
    ) -> Result<IrElement> {
        let empty = HashMap::new();
        let mut elements = vec![
            IrElement::int(size_of(typ, &self.structs) as i32),
            IrElement::Term(IrTerm::Ident(name.clone(), 1)),
        ];
        let mut generator =
            IrFunctionGenerator::new(self.source_code, &empty, &self.structs, &mut self.strings);
        elements.push(generator.expr(expr)?);

        Ok(IrElement::Block(IrBlock {
            name: "var".to_string(),
            elements,
        }))
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

                    elements.push(self.function(&f)?);
                }
                Declaration::Method(typ, f) => {
                    // skip if this function is not used
                    if f.dead_code {
                        continue;
                    }

                    elements.push(self.method(typ, f)?);
                }
                Declaration::Variable(v, expr, t) => {
                    elements.push(self.variable(v, expr, t)?);
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
    (func $main (args)
        (let $int $x 10)
        (assign 1 $x 20)
        (return 1 $x)
    )
)
"#,
            ),
            (
                r#"
func f(c: int): int {
    let x = [1,2,3];
    x[2] = 4;

    return _add(_add(x[1], x[2]), c);
}

func main() {
    return f(10);
}
"#,
                r#"
(module
    (func $f (args $int)
        (let $int $fresh_1 (call $_new 3))
        (assign 1 (offset 1 $fresh_1 0) 1)
        (assign 1 (offset 1 $fresh_1 1) 2)
        (assign 1 (offset 1 $fresh_1 2) 3)
        (let $array $x $fresh_1)

        (assign 1 (offset 1 $x 2) 4)

        (return 1
            (call $_add
                (call $_add
                    (offset 1 $x 1)
                    (offset 1 $x 2))
                $0
            )
        )
    )
    (func $main (args)
        (return 1 (call $f 10))
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
    (func $Point_sum (args $ref)
        (return 1 (call
            $_add
            (offset 1 (deref 1 $0) 1)
            (offset 1 (deref 1 $0) 2)
        ))
    )
    (func $main (args)
        (let (struct 3) $p (data 3 10 20))
        (return 1 (call $Point_sum (address $p(3))))
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
    (func $main (args)
        (return 1 nil)
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
    (func $f (args $int $bool)
        (return 1 $1))
    (func $main (args)
        (return 1 (call $f 0 true))
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
    (func $main (args)
        (let (struct 1) $s (data 2 (string 0)))
        (return 1 (call $_println $s))
    )
)
"#,
            ),
        ];

        for (code, ir_code) in cases {
            let mut compiler = Compiler::new();
            let generated = compiler.compile_ir_nostd(code, "main".to_string()).unwrap();
            info!("{}", generated.show());

            let element = parse_ir(ir_code).unwrap();

            assert_eq!(generated, element, "{}", code);
        }
    }
}
