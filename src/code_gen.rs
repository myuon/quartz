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
    Push(DataType),
    Return(usize),
    Copy(usize),
    Alloc(HeapData),
    Call,
    PAssign,
    Free,
    Deref,
}

#[derive(Debug)]
struct CodeGenerator {
    variables: HashMap<String, usize>,
    codes: Vec<OpCode>,
    stack_count: usize,
    pop_count: usize,
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {
            variables: HashMap::new(),
            codes: vec![],
            stack_count: 0,
            pop_count: 0,
        }
    }

    fn push(&mut self, val: DataType) {
        self.codes.push(OpCode::Push(val));
        self.stack_count += 1;
        self.pop_count += 1;
    }

    fn alloc(&mut self, val: UnsizedDataType) -> Result<()> {
        match val {
            UnsizedDataType::Nil => {
                self.codes.push(OpCode::Alloc(HeapData::Nil));
            }
            UnsizedDataType::String(s) => {
                self.codes.push(OpCode::Alloc(HeapData::String(s)));
            }
            UnsizedDataType::Closure(args, body) => {
                let mut generator = CodeGenerator::new();
                generator.variables = self.variables.clone();
                generator.stack_count = self.stack_count;

                let arity = args.len();
                for a in args {
                    generator.variables.insert(a, self.stack_count);
                    generator.stack_count += 1;
                }

                generator.statements(arity, body)?;

                self.codes
                    .push(OpCode::Alloc(HeapData::Closure(generator.codes)));
            }
        }
        self.stack_count += 1;
        self.pop_count += 1;
        Ok(())
    }

    fn load(&mut self, ident: &String) -> Result<()> {
        let v = self
            .variables
            .get(ident)
            .ok_or(anyhow::anyhow!("Ident {} not found", ident))?;

        self.codes.push(OpCode::Copy(self.stack_count - *v));
        self.stack_count += 1;
        self.pop_count += 1;
        Ok(())
    }

    fn ret(&mut self, arity: usize) {
        // return expr;のときexprがstackに乗っている分があるので1を引いておく
        let pop = self.pop_count + arity;

        self.codes.push(OpCode::Return(pop));
        self.stack_count -= pop;
    }

    fn expr(&mut self, expr: Expr) -> Result<()> {
        match expr {
            Expr::Var(v) => self.load(&v),
            Expr::Lit(lit) => {
                match lit {
                    Literal::Int(n) => self.push(DataType::Int(n)),
                    Literal::String(s) => self.alloc(UnsizedDataType::String(s))?,
                };

                Ok(())
            }
            Expr::Fun(args, body) => self.alloc(UnsizedDataType::Closure(args, body)),
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

                self.load(&f)?;
                self.codes.push(OpCode::Call);

                // call実行後はarityはすべてpopされるのでその分popする数が減る
                self.pop_count -= arity;
                self.stack_count -= arity;

                Ok(())
            }
            Expr::Ref(expr) => match expr.as_ref() {
                Expr::Var(v) => {
                    let p = self
                        .variables
                        .get(v)
                        .ok_or(anyhow::anyhow!("Ident {} not found", v))?
                        .clone();

                    self.push(DataType::StackAddr(self.stack_count - p));

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
        self.pop_count = 0;
        for stmt in stmts {
            match stmt {
                Statement::Let(x, e) => {
                    self.expr(e.clone())?;
                    self.variables.insert(x.clone(), self.stack_count);
                }
                Statement::Return(e) => {
                    self.expr(e.clone())?;
                    self.ret(arity);
                    return Ok(());
                }
                Statement::Expr(e) => {
                    self.expr(e.clone())?;
                }
                Statement::Panic => return Ok(()),
            }
        }

        // returnがない場合はreturn nil;と同等
        self.push(DataType::Nil);
        self.ret(arity);
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
                vec![Push(DataType::Int(10)), Copy(0), Copy(0), Return(3)],
            ),
            (
                r#"let x = 10; return &x;"#,
                vec![
                    Push(DataType::Int(10)),
                    Push(DataType::StackAddr(0)),
                    Return(2),
                ],
            ),
            (
                r#"1; 2; 3; 4;"#,
                vec![
                    Push(DataType::Int(1)),
                    Push(DataType::Int(2)),
                    Push(DataType::Int(3)),
                    Push(DataType::Int(4)),
                    Push(DataType::Nil),
                    Return(5),
                ],
            ),
            (
                r#"let f = fn(a) { return a; }; f(1);"#,
                vec![
                    Alloc(HeapData::Closure(vec![Copy(1), Return(2)])),
                    Push(DataType::Int(1)),
                    Copy(1),
                    Call,
                    Push(DataType::Nil),
                    Return(3),
                ],
            ),
            (
                r#"let x = 10; let f = fn (a,b,c,d,e) { return a; }; f(x,x,x,x,x);"#,
                vec![
                    Push(DataType::Int(10)),
                    Alloc(HeapData::Closure(vec![Copy(5), Return(6)])),
                    Copy(1),
                    Copy(2),
                    Copy(3),
                    Copy(4),
                    Copy(5),
                    Copy(5),
                    Call,
                    Push(DataType::Nil),
                    Return(4),
                ],
            ),
            (
                r#"let x = 0; let f = fn (a) { return *a; };"#,
                vec![
                    Push(DataType::Int(0)),
                    Alloc(HeapData::Closure(vec![Copy(1), Deref, Return(2)])),
                    Push(DataType::Nil),
                    Return(3),
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
