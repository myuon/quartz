use std::collections::HashMap;

use anyhow::{bail, ensure, Result};

use crate::{
    ast::{Expr, Literal, Module, Statement},
    runtime::{DataType, UnsizedDataType},
};

#[derive(PartialEq, Debug, Clone)]
pub enum HeapData {
    Nil,
    String(String),
    Closure(Vec<OpCode>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum OpCode {
    Pop(usize),
    Push(DataType),
    Return(usize),
    Copy(usize),
    Alloc(HeapData),
    Call(usize),
    PAssign,
    Free,
    Deref,
}

struct CodeGenerator {
    variables: HashMap<String, usize>,
    codes: Vec<OpCode>,
    stackCount: usize,
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {
            variables: HashMap::new(),
            codes: vec![],
            stackCount: 0,
        }
    }

    fn push(&mut self, val: DataType) {
        self.codes.push(OpCode::Push(val));
        self.stackCount += 1;
    }

    fn alloc(&mut self, val: UnsizedDataType) {
        match val {
            UnsizedDataType::Nil => self.codes.push(OpCode::Alloc(HeapData::Nil)),
            UnsizedDataType::String(s) => self.codes.push(OpCode::Alloc(HeapData::String(s))),
            // StackAddress, Copyのindexを逆順にしたらここのコンパイル処理を追加する
            UnsizedDataType::Closure(args, body) => todo!(),
        }
        self.stackCount += 1;
    }

    fn load(&mut self, ident: &String) -> Result<()> {
        let v = self
            .variables
            .get(ident)
            .ok_or(anyhow::anyhow!("Ident {} not found", ident))?;

        self.codes.push(OpCode::Copy(self.stackCount - *v));
        self.stackCount += 1;
        Ok(())
    }

    fn expr(&mut self, expr: Expr) -> Result<()> {
        match expr {
            Expr::Var(v) => self.load(&v),
            Expr::Lit(lit) => {
                match lit {
                    Literal::Int(n) => self.push(DataType::Int(n)),
                    Literal::String(s) => self.alloc(UnsizedDataType::String(s)),
                };

                Ok(())
            }
            Expr::Fun(args, body) => Ok(self.alloc(UnsizedDataType::Closure(args, body))),
            Expr::Call(f, args) => {
                let arity = args.len();
                for a in args {
                    self.expr(a)?;
                }

                // 特別な組み込み関数(stack, heapに干渉する必要があるものはここで)

                // ポインタ経由の代入: _passign(p,v) == (*p = v)
                if &f == "_passign" {
                    ensure!(arity == 2, "Expected 2 arguments but {:?} given", arity);
                    self.codes.push(OpCode::PAssign);

                    return Ok(());
                }

                // ヒープ領域の開放
                if &f == "_free" {
                    ensure!(arity == 1, "Expected 1 arguments but {:?} given", arity);
                    self.codes.push(OpCode::Free);

                    return Ok(());
                }

                let faddr = self.load(&f)?;
                self.codes.push(OpCode::Call(arity));

                Ok(faddr)
            }
            Expr::Ref(expr) => match expr.as_ref() {
                Expr::Var(v) => {
                    let p = self
                        .variables
                        .get(v)
                        .ok_or(anyhow::anyhow!("Ident {} not found", v))?;

                    self.push(DataType::StackAddr(*p));
                    Ok(())
                }
                _ => bail!("Cannot take the address of {:?}", expr),
            },
            Expr::Deref(expr) => {
                self.expr(expr.as_ref().clone())?;
                self.codes.push(OpCode::Deref);
                Ok(())
            }
        }
    }

    fn statements(&mut self, arity: usize, stmts: Vec<Statement>) -> Result<()> {
        let mut pop_count = 0;
        for stmt in stmts {
            match stmt {
                Statement::Let(x, e) => {
                    self.expr(e.clone())?;
                    self.variables.insert(x.clone(), self.stackCount);
                    pop_count += 1;
                }
                Statement::Return(e) => {
                    self.expr(e.clone())?;
                    self.codes.push(OpCode::Return(pop_count + arity));
                    return Ok(());
                }
                Statement::Expr(e) => {
                    self.expr(e.clone())?;
                }
                Statement::Panic => return Ok(()),
            }
        }

        // returnがない場合はreturn nil;と同等
        self.codes.push(OpCode::Pop(pop_count + arity));
        self.push(DataType::Nil);
        Ok(())
    }

    fn module(&mut self, module: Module) -> Result<()> {
        self.statements(0, module.0)
    }

    pub fn gen_code(&mut self, module: Module) -> Result<()> {
        self.module(module)
    }
}

pub fn gen_code(module: Module) -> Result<Vec<OpCode>> {
    let mut generator = CodeGenerator::new();
    generator.gen_code(module)?;

    Ok(generator.codes)
}

#[cfg(test)]
mod tests {
    use crate::parser::run_parser;

    use super::*;

    #[test]
    fn test_gen_code() {
        use OpCode::*;

        let cases = vec![
            (
                r#"let x = 10; let y = x; return y;"#,
                vec![Push(DataType::Int(10)), Copy(0), Copy(0), Return(2)],
            ),
            (
                r#"let x = 10; let f = fn (a,b) { return a; }; f(x,x);"#,
                vec![
                    Push(DataType::Int(10)),
                    Alloc(HeapData::Closure(vec![])),
                    Copy(0),
                    Copy(0),
                    Copy(1),
                    Call(2),
                    Pop(2),
                    Push(DataType::Nil),
                ],
            ),
        ];

        for c in cases {
            let m = run_parser(c.0).unwrap();
            let result = gen_code(m).unwrap();
            assert_eq!(result, c.1, "{:?}", c.0);
        }
    }
}
