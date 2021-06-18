use std::collections::HashMap;

use anyhow::{bail, ensure, Result};

use crate::ast::{Expr, Literal, Module, Statement};

#[derive(PartialEq, Debug, Clone)]
pub enum RuntimeData {
    Nil,
    Int(i32),
    String(String),
    Closure(Vec<String>, Vec<Statement>),
}

type FFITable = HashMap<String, Box<fn(Vec<RuntimeData>) -> RuntimeData>>;

struct Interpreter {
    stack: Vec<RuntimeData>,
    variables: HashMap<String, usize>,
    ffi_table: FFITable,
}

impl Interpreter {
    pub fn new(ffi_table: FFITable) -> Interpreter {
        Interpreter {
            stack: vec![],
            variables: HashMap::new(),
            ffi_table,
        }
    }

    fn push(&mut self, val: RuntimeData) -> usize {
        let u = self.stack.len();
        self.stack.push(val);
        u
    }

    fn load(&mut self, ident: String) -> Result<RuntimeData> {
        let v = self
            .variables
            .get(&ident)
            .ok_or(anyhow::anyhow!("Ident {} not found", ident))?;

        Ok(self.stack[*v].clone())
    }

    fn statements(&mut self, stmts: Vec<Statement>) -> Result<RuntimeData> {
        for stmt in stmts {
            match stmt {
                Statement::Let(x, e) => {
                    let val = self.expr(e.clone())?;
                    let p = self.push(val);
                    self.variables.insert(x.clone(), p);
                }
                Statement::Return(e) => {
                    // statementsではreturnしたら以後の部分は評価されない
                    return Ok(self.expr(e.clone())?);
                }
                Statement::Expr(e) => {
                    self.expr(e.clone())?;
                }
            }
        }

        Ok(RuntimeData::Nil)
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

                // ffi
                match self.ffi_table.get(&f) {
                    Some(f) => Ok(f(vargs)),
                    _ => match self.load(f)? {
                        RuntimeData::Closure(vs, body) => {
                            ensure!(
                                vs.len() == arity,
                                "Expected {} arguments but {} given",
                                vs.len(),
                                arity,
                            );

                            let prev = self.variables.clone();
                            for (v, val) in vs.into_iter().zip(vargs) {
                                let p = self.push(val);
                                self.variables.insert(v, p);
                            }

                            let result = self.statements(body)?;

                            self.variables = prev;

                            Ok(result)
                        }
                        r => bail!("Expected closure but found {:?}", r),
                    },
                }
            }
        }
    }

    pub fn module(&mut self, module: Module) -> Result<RuntimeData> {
        self.statements(module.0)
    }
}

pub fn interpret(ffi_table: FFITable, module: Module) -> Result<RuntimeData> {
    let mut interpreter = Interpreter::new(ffi_table);
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
                r#"let x = 10; let f = fn (a,b) { return a; }; return f(x,x);"#,
                RuntimeData::Int(10),
            ),
            (
                // shadowingが起こる例
                r#"let x = 10; let f = fn (x) { let x = 5; return x; }; f(x); return x;"#,
                RuntimeData::Int(10),
            ),
            (
                // 日本語
                r#"let u = "こんにちは、世界"; return u;"#,
                RuntimeData::String("こんにちは、世界".to_string()),
            ),
            (
                // early return
                r#"return 1; return 2; return 3;"#,
                RuntimeData::Int(1),
            ),
            (r#"return _add(1,2);"#, RuntimeData::Int(3)),
        ];

        let mut ffi_table: FFITable = HashMap::new();
        ffi_table.insert(
            "_add".to_string(),
            Box::new(|vs: Vec<RuntimeData>| match (&vs[0], &vs[1]) {
                (RuntimeData::Int(x), RuntimeData::Int(y)) => RuntimeData::Int(x + y),
                _ => todo!(),
            }),
        );
        ffi_table.insert(
            "_minus".to_string(),
            Box::new(|vs: Vec<RuntimeData>| match (&vs[0], &vs[1]) {
                (RuntimeData::Int(x), RuntimeData::Int(y)) => RuntimeData::Int(x - y),
                _ => todo!(),
            }),
        );

        for c in cases {
            let m = run_parser(c.0).unwrap();
            let result = interpret(ffi_table.clone(), m).unwrap();
            assert_eq!(result, c.1, "{:?}", c.0);
        }
    }
}
