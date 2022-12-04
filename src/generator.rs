use std::collections::HashSet;

use anyhow::{bail, Ok, Result};

use crate::ast::{Decl, Expr, Func, Ident, Lit, Module, Statement, Type, VarType};

pub struct Generator {
    pub writer: Writer,
    pub globals: HashSet<Ident>,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            writer: Writer::new(),
            globals: HashSet::new(),
        }
    }

    pub fn set_globals(&mut self, globals: HashSet<Ident>) {
        self.globals = globals;
    }

    pub fn run(&mut self, module: &mut Module) -> Result<()> {
        self.module(module)?;

        Ok(())
    }

    fn module(&mut self, module: &mut Module) -> Result<()> {
        self.writer.start();
        self.writer.write("module");
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
            Decl::Let(ident, type_, expr) => self.global_let(ident, type_, expr),
        }
    }

    fn func(&mut self, func: &mut Func) -> Result<()> {
        self.writer.start();
        self.writer.write("func");
        self.writer.write(&format!("${}", func.name.as_str()));
        for (name, type_) in &mut func.params {
            self.writer
                .write(&format!("(param ${} {})", name.as_str(), type_.to_string()));
        }

        if !func.result.is_nil() {
            self.writer
                .write(&format!("(result {})", func.result.to_string()));
        }

        for statement in &mut func.body {
            if let Statement::Let(ident, type_, _) = statement {
                self.writer.start();
                self.writer
                    .write(&format!("local ${} {}", ident.as_str(), type_.to_string()));
                self.writer.end();
            }
        }
        for statement in &mut func.body {
            self.statement(statement)?;
        }

        self.writer.end();

        Ok(())
    }

    fn global_let(&mut self, ident: &mut Ident, type_: &mut Type, expr: &mut Expr) -> Result<()> {
        self.writer.start();
        self.writer.write("global");
        self.writer.write(&format!("${}", ident.as_str()));
        self.writer.write(&format!("(mut {})", type_.to_string()));
        self.writer.start();
        self.expr(expr)?;
        self.writer.end();
        self.writer.end();

        Ok(())
    }

    fn statement(&mut self, statement: &mut Statement) -> Result<()> {
        match statement {
            Statement::Let(ident, _, value) => {
                self.writer.new_statement();
                self.expr(value)?;

                self.writer.new_statement();
                self.writer.write("local.set");
                self.writer.write(&format!("${}", ident.as_str()));
            }
            Statement::Return(value) => {
                self.writer.new_statement();
                self.expr(value)?;
            }
            Statement::Expr(expr) => {
                self.writer.new_statement();
                self.expr(expr)?;
            }
            Statement::Assign(var_type, lhs, rhs) => {
                self.writer.new_statement();
                self.expr(rhs)?;

                self.writer.new_statement();
                self.writer.write(match var_type {
                    Some(VarType::Local) => "local.set",
                    Some(VarType::Global) => "global.set",
                    _ => bail!("expected var_type, but None"),
                });
                self.writer.write(&format!("${}", lhs.as_str()));
            }
            Statement::If(cond, type_, then_block, else_block) => {
                self.writer.new_statement();
                self.expr(cond)?;

                self.writer.start();
                self.writer.write("if");
                if !type_.is_nil() {
                    self.writer
                        .write(&format!("(result {})", type_.to_string()));
                }

                self.writer.start();
                self.writer.write("then");
                for statement in then_block {
                    self.writer.new_statement();
                    self.statement(statement)?;
                }
                self.writer.end();

                if let Some(else_block) = else_block {
                    self.writer.start();
                    self.writer.write("else");
                    for statement in else_block {
                        self.writer.new_statement();
                        self.statement(statement)?;
                    }
                    self.writer.end();
                }

                self.writer.end();
            }
        }

        Ok(())
    }

    fn expr(&mut self, expr: &mut Expr) -> Result<()> {
        match expr {
            Expr::Lit(lit) => self.lit(lit),
            Expr::Ident(ident) => {
                self.writer.write(if self.globals.contains(ident) {
                    "global.get"
                } else {
                    "local.get"
                });
                self.ident(ident)?;

                Ok(())
            }
            Expr::Call(caller, args) => self.call(caller, args),
        }
    }

    fn lit(&mut self, literal: &mut Lit) -> Result<()> {
        match literal {
            Lit::I32(value) => self.writer.write(&format!("i32.const {}", value)),
        }

        Ok(())
    }

    fn ident(&mut self, ident: &mut Ident) -> Result<()> {
        self.writer.write(&format!("${}", ident.as_str()));

        Ok(())
    }

    fn call(&mut self, caller: &mut Ident, args: &mut Vec<Expr>) -> Result<()> {
        for arg in args {
            self.writer.new_statement();
            self.expr(arg)?;
        }

        self.writer.new_statement();

        match caller.as_str() {
            "add" => {
                self.writer.write("i32.add");
            }
            "sub" => {
                self.writer.write("i32.sub");
            }
            "mult" => {
                self.writer.write("i32.mul");
            }
            "equal" => {
                self.writer.write("i32.eq");
            }
            _ => {
                self.writer.write(&format!("call ${}", caller.as_str()));
            }
        }

        Ok(())
    }
}

pub struct Writer {
    pub buffer: String,
    depth: usize,
    index: usize,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            buffer: String::new(),
            depth: 0,
            index: 0,
        }
    }

    pub fn write(&mut self, text: &str) {
        self.buffer.push_str(&format!(
            "{}{}",
            if self.index == 0 { "" } else { " " },
            text
        ));
        self.index += 1;
    }

    pub fn start(&mut self) {
        self.new_statement();
        self.write("(");
        self.depth += 1;
        self.index = 0;
    }

    pub fn end(&mut self) {
        self.depth -= 1;
        self.index = 0;
        self.write(")");
    }

    pub fn new_statement(&mut self) {
        if self.index != 0 {
            self.write(&format!("\n{}", " ".repeat(self.depth)));
        }
        self.index = 0;
    }

    pub fn finalize(&mut self) {
        for _ in 0..self.depth {
            self.end();
        }
    }
}
