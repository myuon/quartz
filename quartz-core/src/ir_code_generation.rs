use std::{collections::HashMap, fmt::Debug};

use anyhow::Result;

use crate::{
    ast::{Declaration, Expr, Function, Literal, Module, Source, Statement, Structs},
    ir::{IrBlock, IrElement, IrTerm},
};

#[derive(Debug)]
struct IrFunctionGenerator<'s> {
    ir: Vec<IrElement>,
    args: &'s HashMap<String, usize>,
    var_index: usize,
    self_object: Option<IrElement>,
    structs: &'s Structs,
}

impl<'s> IrFunctionGenerator<'s> {
    pub fn new(args: &'s HashMap<String, usize>, structs: &'s Structs) -> IrFunctionGenerator<'s> {
        IrFunctionGenerator {
            ir: vec![],
            args,
            var_index: 0,
            self_object: None,
            structs,
        }
    }

    pub fn var_fresh(&mut self) -> Result<IrTerm> {
        self.var_index += 1;

        Ok(IrTerm::Ident(format!("fresh_{}", self.var_index)))
    }

    pub fn expr(&mut self, expr: &Expr) -> Result<IrElement> {
        match expr {
            Expr::Var(v) => {
                if self.args.contains_key(v) {
                    Ok(IrElement::Term(IrTerm::Argument(self.args[v])))
                } else {
                    Ok(IrElement::Term(IrTerm::Ident(v.clone())))
                }
            }
            Expr::Lit(literal) => match literal {
                Literal::Nil => Ok(IrElement::Term(IrTerm::Nil)),
                Literal::Bool(b) => Ok(IrElement::Term(IrTerm::Bool(*b))),
                Literal::Int(n) => Ok(IrElement::Term(IrTerm::Int(*n))),
                Literal::String(_) => todo!(),
                Literal::Array(arr) => {
                    let v = self.var_fresh()?;
                    let n = arr.len() as i32;

                    // in: [1,2]
                    //
                    // out: (let $v (new 2))
                    //      (assign (padd $v 0) 1)
                    //      (assign (padd $v 1) 2)
                    //      $v
                    self.ir.push(IrElement::block(
                        "let",
                        vec![
                            IrElement::Term(v.clone()),
                            IrElement::instruction(
                                "call",
                                vec![IrTerm::Ident("_new".to_string()), IrTerm::Int(n)],
                            ),
                        ],
                    ));

                    for (i, elem) in arr.into_iter().enumerate() {
                        let velem = self.expr(&elem)?;

                        self.ir.push(IrElement::block(
                            "assign",
                            vec![
                                IrElement::instruction(
                                    "call",
                                    vec![
                                        IrTerm::Ident("_padd".to_string()),
                                        v.clone(),
                                        IrTerm::Int(i as i32),
                                    ],
                                ),
                                velem,
                            ],
                        ))
                    }

                    Ok(IrElement::Term(v))
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

                Ok(IrElement::block("call", elements))
            }
            Expr::Struct(struct_name, exprs) => {
                // in: A { a: 1, b: 2 }
                // out: (let $v (new 2))
                //      (assign (padd $v 0) 1)
                //      (assign (padd $v 1) 2)
                //      $v
                let size = self.structs.size_of_struct(&struct_name);

                let obj = self.var_fresh()?;

                self.ir.push(IrElement::block(
                    "let",
                    vec![
                        IrElement::Term(obj.clone()),
                        IrElement::instruction(
                            "call",
                            vec![IrTerm::Ident("_new".to_string()), IrTerm::Int(size as i32)],
                        ),
                    ],
                ));

                for (label, expr) in exprs {
                    let index = self.structs.get_projection_offset(struct_name, label)?;
                    let v = self.expr(expr)?;

                    self.ir.push(IrElement::block(
                        "assign",
                        vec![
                            IrElement::instruction(
                                "call",
                                vec![
                                    IrTerm::Ident("_padd".to_string()),
                                    obj.clone(),
                                    IrTerm::Int(index as i32),
                                ],
                            ),
                            v,
                        ],
                    ));
                    self.expr(expr)?;
                }

                Ok(IrElement::Term(obj))
            }
            Expr::Project(method, struct_name, proj, label) if *method => {
                self.self_object = Some(self.expr(proj)?);

                Ok(IrElement::Term(IrTerm::Ident(format!(
                    "{}_{}",
                    struct_name, label
                ))))
            }
            Expr::Project(_, struct_name, proj, label) => {
                let index = self.structs.get_projection_offset(struct_name, label)?;

                Ok(IrElement::block(
                    "call",
                    vec![
                        IrElement::Term(IrTerm::Ident("_padd".to_string())),
                        self.expr(proj)?,
                        IrElement::Term(IrTerm::Int(index as i32)),
                    ],
                ))
            }
            Expr::Index(arr, i) => Ok(IrElement::block(
                "call",
                vec![
                    IrElement::Term(IrTerm::Ident("_deref".to_string())),
                    IrElement::block(
                        "call",
                        vec![
                            IrElement::Term(IrTerm::Ident("_padd".to_string())),
                            self.expr(arr.as_ref())?,
                            self.expr(i.as_ref())?,
                        ],
                    ),
                ],
            )),
            Expr::Deref(_) => todo!(),
            Expr::Ref(_) => todo!(),
        }
    }

    pub fn statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Let(x, e) => {
                let v = self.expr(e)?;
                self.ir.push(IrElement::block(
                    "let",
                    vec![IrElement::Term(IrTerm::Ident(x.to_string())), v],
                ));
            }
            Statement::Expr(e) => {
                let x = self.var_fresh()?;
                let v = self.expr(e)?;
                self.ir
                    .push(IrElement::block("let", vec![IrElement::Term(x), v]));
            }
            Statement::Return(e) => {
                let v = self.expr(e)?;
                self.ir.push(IrElement::block("return", vec![v]));
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
            Statement::Continue => todo!(),
            Statement::Assignment(v, e2) => {
                let v2 = self.expr(e2)?;

                match v.as_ref() {
                    Expr::Var(v) => {
                        self.ir.push(IrElement::block(
                            "assign",
                            vec![IrElement::Term(IrTerm::Ident(v.to_string())), v2],
                        ));
                    }
                    Expr::Index(arr, i) => {
                        // in: arr[i] = e2;
                        // out: (assign (padd arr i) e2)

                        let varr = self.expr(arr)?;
                        let vi = self.expr(i)?;
                        self.ir.push(IrElement::block(
                            "assign",
                            vec![
                                IrElement::block(
                                    "call",
                                    vec![
                                        IrElement::Term(IrTerm::Ident("_padd".to_string())),
                                        varr,
                                        vi,
                                    ],
                                ),
                                v2,
                            ],
                        ))
                    }
                    _ => todo!(),
                }
            }
            Statement::Loop(_) => todo!(),
            Statement::While(_, _) => todo!(),
        }

        Ok(())
    }

    fn statements(&mut self, statements: &Vec<Source<Statement>>) -> Result<()> {
        for statement in statements {
            self.statement(&statement.data)?;
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
        ))];
        let mut args = HashMap::new();

        let mut arg_index = 0;
        if let Some((receiver, _, _)) = &function.method_of {
            args.insert(receiver.clone(), 0);
            arg_index += 1;
        }
        for (name, _) in &function.args {
            args.insert(name.clone(), arg_index);
            arg_index += 1;
        }

        elements.push(IrElement::Term(IrTerm::Int(arg_index as i32)));

        let mut generator = IrFunctionGenerator::new(&args, &self.structs);
        generator.statements(&function.body)?;
        elements.extend(generator.ir);

        Ok(IrElement::Block(IrBlock {
            name: "func".to_string(),
            elements,
        }))
    }

    pub fn variable(&mut self, name: &String, expr: &Expr) -> Result<IrElement> {
        let empty = HashMap::new();
        let mut elements = vec![IrElement::Term(IrTerm::Ident(name.clone()))];
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
    (func $main 0
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
    (func $f 1
        (let $fresh_1 (call $_new 3))
        (assign (call $_padd $fresh_1 0) 1)
        (assign (call $_padd $fresh_1 1) 2)
        (assign (call $_padd $fresh_1 2) 3)
        (let $x $fresh_1)

        (assign (call $_padd $x 2) 4)

        (return (call $_add
            (call $_add
                (call $_deref (call $_padd $x 1))
                (call $_deref (call $_padd $x 2))
            )
            $0
        ))
    )
    (func $main 0
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
    (func $Point_sum 1
        (return (call
            $_add
            (call $_padd $0 0)
            (call $_padd $0 1)
        ))
    )
    (func $main 0
        (let $fresh_1 (call $_new 2))
        (assign (call $_padd $fresh_1 0) 10)
        (assign (call $_padd $fresh_1 1) 20)

        (let $p $fresh_1)

        (return (call $Point_sum $p))
    )
)
"#,
            ),
        ];

        for (code, ir_code) in cases {
            let mut compiler = Compiler::new();
            let generated = compiler.compile_ir_nostd(code).unwrap();
            println!("{}", generated.show());

            let element = parse_ir(ir_code).unwrap();

            assert_eq!(generated, element);
        }
    }
}
