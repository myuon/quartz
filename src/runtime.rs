use std::collections::HashMap;

use anyhow::{bail, ensure, Result};

use crate::ast::{Expr, Literal, Module, Statement};

#[derive(PartialEq, Debug, Clone)]
pub enum RuntimeData {
    Nil,
    Int(i32),
    String(String),
    Closure(Vec<String>, Vec<Expr>),
}

struct Interpreter {
    variables: HashMap<String, RuntimeData>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    fn load(&mut self, ident: String) -> Result<RuntimeData> {
        let v = self
            .variables
            .get(&ident)
            .ok_or(anyhow::anyhow!("Ident {} not found", ident))?;

        Ok(v.clone())
    }

    fn expr(&mut self, expr: Expr) -> Result<RuntimeData> {
        match expr {
            Expr::Var(v) => self.load(v),
            Expr::Lit(lit) => Ok(match lit {
                Literal::Int(n) => RuntimeData::Int(n),
                Literal::String(s) => RuntimeData::String(s),
            }),
            Expr::Fun(args, body) => Ok(RuntimeData::Closure(args, body)),
            Expr::Call(f, args) => {
                let arity = args.len();
                let mut vargs = vec![];
                for a in args {
                    vargs.push(self.expr(a)?);
                }

                match self.load(f)? {
                    RuntimeData::Closure(vs, body) => {
                        ensure!(
                            vs.len() == arity,
                            "Expected {} arguments but {} given",
                            vs.len(),
                            arity,
                        );

                        let prev = self.variables.clone();
                        for (v, val) in vs.into_iter().zip(vargs) {
                            self.variables.insert(v, val);
                        }

                        let result = self.exprs(body)?;

                        self.variables = prev;

                        Ok(result)
                    }
                    r => bail!("Expected closure but found {:?}", r),
                }
            }
            Expr::Statement(s) => match s.as_ref() {
                Statement::Let(x, e) => {
                    let val = self.expr(e.clone())?;
                    self.variables.insert(x.clone(), val);

                    Ok(RuntimeData::Nil)
                }
                Statement::Expr(e) => {
                    self.expr(e.clone())?;

                    Ok(RuntimeData::Nil)
                }
            },
        }
    }

    fn exprs(&mut self, exprs: Vec<Expr>) -> Result<RuntimeData> {
        let mut result = RuntimeData::Nil;
        for expr in exprs {
            result = self.expr(expr)?;
        }

        Ok(result)
    }

    pub fn module(&mut self, module: Module) -> Result<RuntimeData> {
        self.exprs(module.0)
    }
}

pub fn interpret(module: Module) -> Result<RuntimeData> {
    let mut interpreter = Interpreter::new();
    interpreter.module(module)
}

#[cfg(test)]
mod tests {
    use crate::parser::run_parser;

    use super::*;

    #[test]
    fn test_runtime() {
        let cases = vec![
            (
                r#"let x = 10; let f = fn (a,b) { a }; f(x,x)"#,
                RuntimeData::Int(10),
            ),
            (
                // shadowingが起こる例
                r#"let x = 10; let f = fn (x) { let x = 5; x }; f(x); x"#,
                RuntimeData::Int(10),
            ),
            (
                // 日本語
                r#"let u = "こんにちは、世界"; u"#,
                RuntimeData::String("こんにちは、世界".to_string()),
            ),
        ];

        for c in cases {
            let m = run_parser(c.0).unwrap();
            let result = interpret(m).unwrap();
            assert_eq!(result, c.1, "{:?}", c.0);
        }
    }
}
