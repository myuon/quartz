use std::collections::HashMap;

use anyhow::Result;

use crate::ast::{Expr, Literal, Module, Statement};

/*
#[derive(PartialEq, Debug)]
pub enum OpCode {
    PushInt(i32),
    PushString(String),
    PushClosure(Vec<String>, Vec<OpCode>),
    Copy(usize),
    Call(usize),
    Pop(),
}

struct CodeGenerator {
    letCount: usize,
    addressTable: HashMap<String, usize>,
}

impl CodeGenerator {
    pub fn new(table: HashMap<String, usize>) -> CodeGenerator {
        CodeGenerator {
            letCount: 0,
            addressTable: table,
        }
    }

    fn expr(&mut self, expr: Expr) -> Result<Vec<OpCode>> {
        let mut codes = vec![];

        match expr {
            Expr::Var(v) => {
                let addr = self
                    .addressTable
                    .get(&v)
                    .ok_or(anyhow::anyhow!("Ident {} not found", v))?;
                codes.push(OpCode::Copy(*addr));
            }
            Expr::Lit(lit) => match lit {
                Literal::Int(n) => {
                    codes.push(OpCode::PushInt(n));
                }
                Literal::String(s) => {
                    codes.push(OpCode::PushString(s));
                }
            },
            Expr::Fun(args, exprs) => {
                // 仮引数の処理
                for (i, arg) in args.iter().enumerate() {
                    self.addressTable.insert(arg.clone(), i + 1);
                }

                let mut body = vec![];
                for expr in exprs {
                    // !!recursion
                    body.extend(self.expr(expr)?);
                }

                codes.push(OpCode::PushClosure(args, body));
            }
            Expr::Call(f, body) => {
                let arity = body.len();
                for expr in body {
                    // !!recursion
                    codes.extend(self.expr(expr)?);
                }

                let addr = self
                    .addressTable
                    .get(&f)
                    .ok_or(anyhow::anyhow!("Ident {} not found", f))?;
                codes.push(OpCode::Copy(*addr));
                codes.push(OpCode::Call(arity));
            }
            Expr::Statement(st) => match st.as_ref() {
                Statement::Let(v, e) => {
                    // !!recursion
                    codes.extend(self.expr(e.clone())?);

                    self.addressTable.insert(v.clone(), self.letCount);
                    self.letCount += 1;
                }
                Statement::Expr(e) => {
                    // !!recursion
                    codes.extend(self.expr(e.clone())?);
                    codes.push(OpCode::Pop());
                }
                Statement::Return(_) => todo!(),
            },
        }

        Ok(codes)
    }

    fn module(&mut self, module: Module) -> Result<Vec<OpCode>> {
        let mut codes = vec![];

        for expr in module.0 {
            codes.extend(self.expr(expr)?);
        }

        Ok(codes)
    }

    pub fn gen_code(&mut self, module: Module) -> Result<Vec<OpCode>> {
        self.module(module)
    }
}

pub fn gen_code(module: Module, ffi_table: HashMap<String, usize>) -> Result<Vec<OpCode>> {
    let mut generator = CodeGenerator::new(ffi_table);

    generator.gen_code(module)
}

#[cfg(test)]
mod tests {
    use crate::parser::run_parser;

    use super::*;

    //#[test]
    fn test_gen_code() {
        use OpCode::*;

        let cases = vec![(
            r#"let x = 10; let f = fn (a,b) { return a; }; f(x,x);"#,
            vec![
                PushInt(10),
                PushClosure(vec!["a".to_string(), "b".to_string()], vec![Copy(1)]),
                Copy(0),
                Copy(0),
                Copy(1),
                Call(2),
            ],
        )];

        let mut table = HashMap::new();
        table.insert("_add".to_string(), 55);

        for c in cases {
            let m = run_parser(c.0).unwrap();
            let result = gen_code(m, table.clone()).unwrap();
            assert_eq!(result, c.1, "{:?}", c.0);
        }
    }
}
 */
