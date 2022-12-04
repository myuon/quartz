use anyhow::Result;

use crate::ast::{Decl, Expr, Func, Ident, Lit, Module, Statement};

pub struct Generator {
    pub writer: Writer,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            writer: Writer::new(),
        }
    }

    pub fn run(&mut self, module: &mut Module) -> Result<()> {
        self.module(module)?;

        Ok(())
    }

    fn module(&mut self, module: &mut Module) -> Result<()> {
        self.writer.start();
        self.writer.write("module ");
        for decl in &mut module.0 {
            self.decl(decl)?;
        }

        self.writer.start();
        self.writer.write(r#"export "main" (func $main)"#);
        self.writer.end();
        self.writer.end();

        self.writer.finalize();

        Ok(())
    }

    fn decl(&mut self, decl: &mut Decl) -> Result<()> {
        match decl {
            Decl::Func(func) => self.func(func),
        }
    }

    fn func(&mut self, func: &mut Func) -> Result<()> {
        self.writer.start();
        self.writer.write("func ");
        self.writer.write(&format!("${} ", func.name.as_str()));
        for (name, type_) in &mut func.params {
            self.writer
                .write(&format!("(param ${} {}) ", name.as_str(), type_.as_str()));
        }
        self.writer
            .write(&format!("(result {}) ", func.result.as_str()));
        for statement in &mut func.body {
            self.statement(statement)?;
        }
        self.writer.end();

        Ok(())
    }

    fn statement(&mut self, statement: &mut Statement) -> Result<()> {
        match statement {
            Statement::Let(ident, type_, value) => {
                self.writer.start();
                self.writer
                    .write(&format!("local ${} {}", ident.as_str(), type_.as_str()));
                self.writer.end();

                self.writer.start();
                self.writer.write("local.set ");
                self.writer.write(&format!("${} ", ident.as_str()));
                self.expr(value)?;
                self.writer.write("");
                self.writer.end();
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
            Lit::I32(value) => self.writer.write(&format!("i32.const {} ", value)),
        }

        Ok(())
    }

    fn ident(&mut self, ident: &mut Ident) -> Result<()> {
        match ident.as_str() {
            "add" => {
                self.writer.write("i32.add ");
            }
            _ => {
                self.writer
                    .write(&format!("local.get ${} ", ident.as_str()));
            }
        }

        Ok(())
    }

    fn call(&mut self, caller: &mut Expr, args: &mut Vec<Expr>) -> Result<()> {
        for arg in args {
            self.writer.new_statement();
            self.expr(arg)?;
        }

        self.writer.new_statement();
        self.expr(caller)?;

        Ok(())
    }
}

pub struct Writer {
    pub buffer: String,
    depth: usize,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            buffer: String::new(),
            depth: 0,
        }
    }

    pub fn write(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    pub fn start(&mut self) {
        self.write(&format!("\n{}(", " ".repeat(self.depth)));
        self.depth += 1;
    }

    pub fn end(&mut self) {
        self.write(") ");
        self.depth -= 1;
    }

    pub fn new_statement(&mut self) {
        self.write(&format!("\n{}", " ".repeat(self.depth)));
    }

    pub fn finalize(&mut self) {
        for _ in 0..self.depth {
            self.end();
        }
    }
}
