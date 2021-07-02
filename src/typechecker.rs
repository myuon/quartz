use std::collections::HashMap;

use anyhow::Result;

use crate::ast::{Expr, Module, Statement};

pub struct TypeChecker {
    local: Vec<String>,
    outer_variables: Vec<String>,
    // closureで使われる外部変数を集めたもの
    closures: HashMap<usize, Vec<String>>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            local: vec![],
            outer_variables: vec![],
            closures: HashMap::new(),
        }
    }

    fn expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Var(v) => {
                // _から始まるやつはreservedということにして一旦スルーする
                if !self.local.contains(v)
                    && !self.outer_variables.contains(v)
                    && !v.starts_with("_")
                {
                    self.outer_variables.push(v.clone());
                }
            }
            Expr::Lit(_) => {}
            Expr::Fun(id, args, body) => {
                let local = self.local.clone();
                self.local = args.clone();
                self.outer_variables = vec![];
                self.statements(body)?;
                self.closures.insert(*id, self.outer_variables.clone());
                self.local = local;
            }
            Expr::Call(_, e, es) => {
                self.expr(e)?;
                for e in es {
                    self.expr(e)?;
                }
            }
            Expr::Ref(e) => {
                self.expr(e)?;
            }
            Expr::Deref(e) => {
                self.expr(e)?;
            }
            Expr::Loop(body) => {
                self.statements(body)?;
            }
        }

        Ok(())
    }

    fn statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Let(x, e) => {
                self.expr(e)?;
                self.local.push(x.clone());
            }
            Statement::Expr(e) => {
                self.expr(e)?;
            }
            Statement::Return(e) => {
                self.expr(e)?;
            }
            Statement::ReturnIf(e1, e2) => {
                self.expr(e1)?;
                self.expr(e2)?;
            }
            Statement::Panic => {}
        }

        Ok(())
    }

    fn statements(&mut self, statements: &Vec<Statement>) -> Result<()> {
        for stmt in statements {
            self.statement(stmt)?;
        }

        Ok(())
    }

    fn module(&mut self, module: &Module) -> Result<()> {
        self.statements(&module.0)?;

        Ok(())
    }

    pub fn check(&mut self, module: &Module) -> Result<()> {
        self.module(module)
    }
}

pub fn typechecker(module: &Module) -> Result<HashMap<usize, Vec<String>>> {
    let mut t = TypeChecker::new();
    t.check(module)?;

    Ok(t.closures)
}
