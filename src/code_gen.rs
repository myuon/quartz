use std::collections::HashMap;

use anyhow::{bail, ensure, Result};

use crate::{
    ast::{Expr, Literal, Module, Statement},
    runtime::{DataType, UnsizedDataType},
};

#[derive(PartialEq, Debug)]
pub enum OpCode {
    Pop(usize),
    Push(DataType),
    Return(usize),
    Copy(usize),
    Alloc(UnsizedDataType),
    Call(usize),
    PAssign(),
    Free(),
    Deref(),
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

    fn push(&mut self, val: DataType) -> usize {
        let p = self.stackCount;
        self.codes.push(OpCode::Push(val));
        self.stackCount += 1;

        p
    }

    fn alloc(&mut self, val: UnsizedDataType) -> usize {
        let p = self.stackCount;
        self.stackCount += 1;
        self.codes.push(OpCode::Alloc(val));

        p
    }

    fn load(&mut self, ident: &String) -> Result<usize> {
        let v = self
            .variables
            .get(ident)
            .ok_or(anyhow::anyhow!("Ident {} not found", ident))?;

        let p = self.stackCount;
        self.codes.push(OpCode::Copy(*v));
        self.stackCount += 1;
        Ok(p)
    }

    fn expr(&mut self, expr: Expr) -> Result<usize> {
        match expr {
            Expr::Var(v) => self.load(&v),
            Expr::Lit(lit) => Ok(match lit {
                Literal::Int(n) => self.push(DataType::Int(n)),
                Literal::String(s) => self.alloc(UnsizedDataType::String(s)),
            }),
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
                    self.codes.push(OpCode::PAssign());

                    return Ok(self.stackCount);
                }

                // ヒープ領域の開放
                if &f == "_free" {
                    ensure!(arity == 1, "Expected 1 arguments but {:?} given", arity);
                    self.codes.push(OpCode::Free());

                    return Ok(self.stackCount);
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

                    Ok(self.push(DataType::StackAddr(*p)))
                }
                _ => bail!("Cannot take the address of {:?}", expr),
            },
            Expr::Deref(expr) => {
                self.expr(expr.as_ref().clone())?;
                self.codes.push(OpCode::Deref());
                Ok(self.stackCount)
            }
        }
    }

    fn statements(&mut self, arity: usize, stmts: Vec<Statement>) -> Result<()> {
        let mut pop_count = 0;
        for stmt in stmts {
            match stmt {
                Statement::Let(x, e) => {
                    let p = self.expr(e.clone())?;
                    self.variables.insert(x.clone(), p);
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

        let cases = vec![(
            r#"let x = 10; let f = fn (a,b) { return a; }; f(x,x);"#,
            vec![
                Push(DataType::Int(10)),
                Alloc(UnsizedDataType::Closure(
                    vec!["a".to_string(), "b".to_string()],
                    vec![Statement::Return(Expr::Var("a".to_string()))],
                )),
                Copy(0),
                Copy(0),
                Copy(1),
                Call(2),
                Pop(2),
                Push(DataType::Nil),
            ],
        )];

        for c in cases {
            let m = run_parser(c.0).unwrap();
            let result = gen_code(m).unwrap();
            assert_eq!(result, c.1, "{:?}", c.0);
        }
    }
}
