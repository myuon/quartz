use std::{collections::HashMap, fmt::Debug};

use anyhow::Result;

use crate::{
    ast::{Declaration, Expr, Function, Literal, Module, Statement, Structs},
    ir::{IrBlock, IrElement, IrTerm},
};

#[derive(Debug)]
struct IrFunctionGenerator<'s> {
    ir: Vec<IrElement>,
    args: HashMap<String, usize>,
    var_index: usize,
    self_object: Option<IrElement>,
    structs: &'s Structs,
}

impl IrFunctionGenerator<'_> {
    pub fn new(args: HashMap<String, usize>, structs: &Structs) -> IrFunctionGenerator {
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
                            IrElement::instruction("new", vec![IrTerm::Int(n)]),
                        ],
                    ));

                    for (i, elem) in arr.into_iter().enumerate() {
                        let velem = self.expr(&elem)?;

                        self.ir.push(IrElement::block(
                            "assign",
                            vec![
                                IrElement::instruction(
                                    "padd",
                                    vec![v.clone(), IrTerm::Int(i as i32)],
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

                for arg in args {
                    elements.push(self.expr(&arg)?);
                }

                Ok(IrElement::block("call", elements))
            }
            Expr::Struct(_, _) => todo!(),
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
                    "padd",
                    vec![self.expr(proj)?, IrElement::Term(IrTerm::Int(index as i32))],
                ))
            }
            Expr::Index(arr, i) => Ok(IrElement::block(
                "padd",
                vec![self.expr(arr.as_ref())?, self.expr(i.as_ref())?],
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
            Statement::If(_, _, _) => todo!(),
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
                            vec![IrElement::block("padd", vec![varr, vi]), v2],
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
        let mut elements = vec![IrElement::Term(IrTerm::Ident(function.name.clone()))];
        let mut args = HashMap::new();

        for (index, (name, _)) in function.args.iter().enumerate() {
            args.insert(name.clone(), index);
        }

        let mut generator = IrFunctionGenerator::new(args, &self.structs);
        for b in &function.body {
            generator.statement(&b.data)?;
        }
        elements.extend(generator.ir);

        Ok(IrElement::Block(IrBlock {
            name: "func".to_string(),
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
                Declaration::Variable(_, _) => todo!(),
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

    use super::*;

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
    (func $main
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
    (func $f
        (let $fresh_1 (new 3))
        (assign (padd $fresh_1 0) 1)
        (assign (padd $fresh_1 1) 2)
        (assign (padd $fresh_1 2) 3)
        (let $x $fresh_1)

        (assign (padd $x 2) 4)

        (return (call $_add
            (call $_add
                (padd $x 1)
                (padd $x 2)
            )
            $0
        ))
    )
    (func $main
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
    (func $Point_sum
        (return (call
            $_add
            (padd $0 0)
            (padd $0 1)
        ))
    )
    (func $main
        (let $p (new 2))
        (assign (padd $p 0) 10)
        (assign (padd $p 1) 20)

        (return (call $Point_sum $p))
    )
)
"#,
            ),
        ];

        for (code, ir_code) in cases {
            let mut compiler = Compiler::new();
            let generated = compiler.compile_ir(code).unwrap();

            let element = parse_ir(ir_code).unwrap();

            assert_eq!(generated, element);
        }
    }
}
