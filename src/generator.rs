use anyhow::Result;

use crate::ast::{Decl, Expr, Func, Ident, Lit, Module, Statement};

pub struct Generator {
    pub writer: String,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            writer: String::new(),
        }
    }

    pub fn run(&mut self, module: &mut Module) -> Result<()> {
        self.module(module)?;

        Ok(())
    }

    fn module(&mut self, module: &mut Module) -> Result<()> {
        self.writer.push_str("(module\n");
        for decl in &mut module.0 {
            self.decl(decl)?;
        }

        self.writer.push_str(r#"(export "main" (func $main))"#);
        self.writer.push_str(")");

        Ok(())
    }

    fn decl(&mut self, decl: &mut Decl) -> Result<()> {
        match decl {
            Decl::Func(func) => self.func(func),
        }
    }

    fn func(&mut self, func: &mut Func) -> Result<()> {
        self.writer.push_str("(func\n");
        self.writer.push_str(&format!("${} ", func.name.as_str()));
        for (name, type_) in &mut func.params {
            self.writer
                .push_str(&format!("(param ${} {})", name.as_str(), type_.as_str()));
        }
        self.writer
            .push_str(&format!("(result {})", func.result.as_str()));
        for statement in &mut func.body {
            self.statement(statement)?;
        }
        self.writer.push_str(")");

        Ok(())
    }

    fn statement(&mut self, statement: &mut Statement) -> Result<()> {
        match statement {
            Statement::Let(ident, type_, value) => {
                self.writer.push_str("(local ");
                self.writer
                    .push_str(&format!("${} {})", ident.as_str(), type_.as_str()));

                self.writer.push_str("(local.set ");
                self.writer.push_str(&format!("${} ", ident.as_str()));
                self.expr(value)?;
                self.writer.push_str(") ");
            }
            Statement::Return(value) => {
                self.expr(value)?;
            }
        }

        Ok(())
    }

    fn expr(&mut self, expr: &mut Expr) -> Result<()> {
        match expr {
            Expr::Lit(lit) => self.lit(lit),
            Expr::Ident(ident) => self.ident(ident),
            Expr::Call(caller, args) => self.call(caller, args),
        }
    }

    fn lit(&mut self, literal: &mut Lit) -> Result<()> {
        match literal {
            Lit::I32(value) => self.writer.push_str(&format!("i32.const {} ", value)),
        }

        Ok(())
    }

    fn ident(&mut self, ident: &mut Ident) -> Result<()> {
        match ident.as_str() {
            "add" => self.writer.push_str("i32.add "),
            _ => self
                .writer
                .push_str(&format!("local.get ${} ", ident.as_str())),
        }

        Ok(())
    }

    fn call(&mut self, caller: &mut Expr, args: &mut Vec<Expr>) -> Result<()> {
        for arg in args {
            self.expr(arg)?;
        }
        self.expr(caller)?;

        Ok(())
    }
}
