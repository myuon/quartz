use std::{collections::HashMap, fmt::Debug};

use anyhow::{bail, Context, Result};

use crate::{
    ast::{
        size_of, Declaration, Expr, Function, Literal, Module, Source, Statement, Structs, Type,
    },
    ir::{IrBlock, IrElement, IrTerm},
};

#[derive(Debug)]
struct IrFunctionGenerator<'s> {
    ir: Vec<IrElement>,
    args: &'s HashMap<String, usize>,
    fresh_var_index: usize,
    self_object: Option<IrElement>,
    structs: &'s Structs,
}

impl<'s> IrFunctionGenerator<'s> {
    pub fn new(args: &'s HashMap<String, usize>, structs: &'s Structs) -> IrFunctionGenerator<'s> {
        IrFunctionGenerator {
            ir: vec![],
            args,
            fresh_var_index: 0,
            self_object: None,
            structs,
        }
    }

    fn stack_size_of(&self, ty: &Type) -> Result<usize> {
        Ok(match ty {
            Type::Unit => 1,
            Type::Bool => 1,
            Type::Int => 1,
            Type::Byte => 1,
            Type::Struct(s) => self.structs.size_of_struct(s.as_str()),
            Type::Array(_) => 1,
            Type::Fn(_, _) => 1,
            _ => bail!("Unsupported type: {:?}", ty),
        })
    }

    pub fn var_fresh(&mut self) -> String {
        self.fresh_var_index += 1;

        format!("fresh_{}", self.fresh_var_index)
    }

    pub fn expr(&mut self, expr: &Source<Expr>) -> Result<IrElement> {
        match &expr.data {
            Expr::Var(v, _) => {
                if self.args.contains_key(v) {
                    Ok(IrElement::Term(IrTerm::Argument(self.args[v], 1)))
                } else {
                    Ok(IrElement::Term(IrTerm::Ident(v.clone(), 1)))
                }
            }
            Expr::Lit(literal) => match literal {
                Literal::Nil => Ok(IrElement::Term(IrTerm::Nil)),
                Literal::Bool(b) => Ok(IrElement::Term(IrTerm::Bool(*b))),
                Literal::Int(n) => Ok(IrElement::Term(IrTerm::Int(*n))),
                Literal::String(s) => self.expr(&Source::unknown(Expr::Struct(
                    "string".to_string(),
                    vec![(
                        "data".to_string(),
                        Source::unknown(Expr::Lit(Literal::Array(
                            s.as_bytes()
                                .iter()
                                .map(|i| Source::unknown(Expr::Lit(Literal::Int(*i as i32))))
                                .collect::<Vec<_>>(),
                            Type::Int,
                        ))),
                    )],
                ))),
                Literal::Array(arr, t) => {
                    let size = self
                        .stack_size_of(&Type::Array(Box::new(t.clone())))
                        .context(format!("{:?}", expr))?;
                    let v = self.var_fresh();
                    let n = arr.len() as i32;

                    // in: [1,2]
                    //
                    // out: (let $v (new 2))
                    //      (assign (padd $v 0) 1)
                    //      (assign (padd $v 1) 2)
                    //      $v
                    self.ir.push(IrElement::i_let(
                        IrElement::ir_type(t),
                        size,
                        v.clone(),
                        IrElement::i_call("_new", vec![IrElement::int(n)]),
                    ));

                    for (i, elem) in arr.into_iter().enumerate() {
                        let velem = self.expr(&elem)?;

                        self.ir.push(IrElement::i_assign(
                            IrElement::i_call(
                                "_padd",
                                vec![
                                    IrElement::Term(IrTerm::Ident(v.clone(), size)),
                                    IrElement::int(i as i32),
                                ],
                            ),
                            velem,
                        ));
                    }

                    Ok(IrElement::Term(IrTerm::Ident(v, size)))
                }
            },
            Expr::Call(f, args) => {
                // in: f(a,b,c)
                // out: (call f a b c)
                let mut elements = vec![];
                elements.push(self.expr(f.as_ref())?);

                if let Some(self_obj) = self.self_object.take() {
                    elements.push(self_obj);
                }

                for arg in args {
                    elements.push(self.expr(&arg)?);
                }

                Ok(IrElement::i_call_raw(elements))
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
                for (_label, expr) in exprs {
                    let v = self.expr(expr)?;
                    data.push(v);
                }

                Ok(IrElement::block("data", data))
            }
            Expr::Project(method, struct_name, proj, label) if *method => {
                self.self_object = Some(self.expr(proj)?);

                Ok(IrElement::Term(IrTerm::Ident(
                    format!("{}_{}", struct_name, label),
                    1,
                )))
            }
            Expr::Project(_, struct_name, proj, label) => {
                let index = self.structs.get_projection_offset(struct_name, label)?;
                let is_addr = self
                    .structs
                    .get_projection_type(struct_name, label)?
                    .is_addr_type();

                let block = IrElement::i_call(
                    "_padd",
                    vec![
                        self.expr(proj)?,
                        IrElement::Term(IrTerm::Int(index as i32)), // back to the first work of the struct
                    ],
                );

                Ok(
                    // deref if the block is addr
                    if !is_addr {
                        IrElement::i_call("_deref", vec![block])
                    } else {
                        block
                    },
                )
            }
            Expr::Index(arr, i) => Ok(IrElement::i_call(
                "_deref",
                vec![IrElement::i_call(
                    "_padd",
                    vec![self.expr(arr.as_ref())?, self.expr(i.as_ref())?],
                )],
            )),
        }
    }

    fn statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Let(x, e, t) => {
                let v = self.expr(e)?;
                self.ir.push(IrElement::i_let(
                    IrElement::ir_type(t),
                    size_of(t, self.structs),
                    x.to_string(),
                    v,
                ));
            }
            Statement::Expr(e, t) => {
                let v = self.expr(e)?;
                self.ir.push(v);
                self.ir.push(IrElement::instruction(
                    "pop",
                    vec![IrTerm::Int(self.stack_size_of(t)? as i32)],
                ));
            }
            Statement::Return(e, t) => {
                let v = self.expr(e)?;
                self.ir.push(IrElement::block(
                    "return",
                    vec![
                        IrElement::Term(IrTerm::Int(
                            self.stack_size_of(t).context(format!("{:?}", statement))? as i32,
                        )),
                        v,
                    ],
                ));
            }
            Statement::If(b, s1, s2) => {
                let v = self.expr(b)?;
                let gen1 = {
                    let mut generator = IrFunctionGenerator::new(self.args, &self.structs);
                    generator.statements(&s1)?;
                    generator.ir
                };
                let gen2 = {
                    let mut generator = IrFunctionGenerator::new(self.args, &self.structs);
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
            Statement::Assignment(v, e2) => {
                match &v.data {
                    Expr::Var(v, _) => {
                        let v2 = self.expr(e2)?;
                        self.ir.push(IrElement::i_assign(
                            IrElement::Term(IrTerm::Ident(v.to_string(), 1)),
                            v2,
                        ));
                    }
                    Expr::Index(arr, i) => {
                        // in: arr[i] = e2;
                        // out: (assign (padd arr i) e2)

                        let v2 = self.expr(e2)?;
                        let varr = self.expr(arr)?;
                        let vi = self.expr(i)?;
                        self.ir.push(IrElement::i_assign(
                            IrElement::i_call("_padd", vec![varr, vi]),
                            v2,
                        ))
                    }
                    Expr::Project(false, struct_name, proj, label) => {
                        let index = self.structs.get_projection_offset(struct_name, label)?;
                        let v = self.expr(proj)?;
                        let v2 = self.expr(e2)?;

                        self.ir.push(IrElement::i_assign(
                            IrElement::i_call(
                                "_padd",
                                vec![
                                    v,
                                    IrElement::Term(IrTerm::Int(index as i32)), // back to the first work of the struct
                                ],
                            ),
                            v2,
                        ));
                    }
                    _ => todo!(),
                }
            }
            Statement::Loop(_) => todo!(),
            Statement::While(cond, body) => {
                let vcond = self.expr(cond)?;
                let gen = {
                    let mut generator = IrFunctionGenerator::new(self.args, &self.structs);
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
                Source::unknown(Expr::Lit(Literal::Nil)),
                Type::Unit,
            ))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct IrGenerator {
    structs: Structs,
}

impl IrGenerator {
    pub fn new() -> IrGenerator {
        IrGenerator {
            structs: Structs(HashMap::new()),
        }
    }

    pub fn context(&mut self, structs: Structs) {
        self.structs = structs;
    }

    pub fn function(&mut self, function: &Function) -> Result<IrElement> {
        let mut elements = vec![IrElement::Term(IrTerm::Ident(
            if let Some((_, struct_name, _)) = &function.method_of {
                format!("{}_{}", struct_name, function.name)
            } else {
                function.name.clone()
            },
            1,
        ))];
        let mut args = HashMap::new();
        let mut arg_index = 0;
        let mut arg_types_in_ir = vec![];

        // argument in reverse order
        for (name, typ) in function.args.iter().rev() {
            arg_index += 1; // self.stack_size_of(typ)?;
            args.insert(name.clone(), arg_index - 1);
            arg_types_in_ir.push(IrElement::ir_type(typ));
        }
        if let Some((receiver, struct_name, _)) = &function.method_of {
            arg_index += 1; // self.stack_size_of(&Type::Struct(struct_name.clone()))?;
            args.insert(receiver.clone(), arg_index - 1);
            arg_types_in_ir.push(IrElement::ir_type(&Type::Struct(struct_name.clone())));
        }

        elements.push(IrElement::block(
            "args",
            arg_types_in_ir.into_iter().rev().collect(),
        ));

        let mut generator = IrFunctionGenerator::new(&args, &self.structs);

        generator.function(&function.body)?;

        elements.extend(generator.ir);

        Ok(IrElement::Block(IrBlock {
            name: "func".to_string(),
            elements,
        }))
    }

    pub fn variable(&mut self, name: &String, expr: &Source<Expr>) -> Result<IrElement> {
        let empty = HashMap::new();
        let mut elements = vec![IrElement::Term(IrTerm::Ident(name.clone(), 1))];
        let mut generator = IrFunctionGenerator::new(&empty, &self.structs);
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
                Declaration::Variable(v, expr) => {
                    elements.push(self.variable(v, expr)?);
                }
                Declaration::Struct(_) => {}
            }
        }

        Ok(IrElement::Block(IrBlock {
            name: "module".to_string(),
            elements,
        }))
    }

    pub fn generate(&mut self, module: &Module) -> Result<IrElement> {
        self.module(module)
    }
}

pub fn generate(module: &Module) -> Result<IrElement> {
    let mut g = IrGenerator::new();
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
        (let $int 1 $x 10)
        (assign $x 20)
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
        (let $int 1 $fresh_1 (call $_new 3))
        (assign (call $_padd $fresh_1 0) 1)
        (assign (call $_padd $fresh_1 1) 2)
        (assign (call $_padd $fresh_1 2) 3)
        (let $array 1 $x $fresh_1)

        (assign (call $_padd $x 2) 4)

        (return 1
            (call $_add
                (call $_add
                    (call $_deref (call $_padd $x 1))
                    (call $_deref (call $_padd $x 2)))
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

func (p: Point) sum(): int {
    return _add(p.x, p.y);
}

func main() {
    let p = Point { x: 10, y: 20 };

    return p.sum();
}
"#,
                r#"
(module
    (func $Point_sum (args $addr)
        (return 1 (call
            $_add
            (call $_deref (call $_padd $0 1))
            (call $_deref (call $_padd $0 2))
        ))
    )
    (func $main (args)
        (let $addr 3 $p(1) (data 3 10 20))
        (return 1 (call $Point_sum $p))
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
