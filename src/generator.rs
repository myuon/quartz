use std::collections::{HashMap, HashSet};

use anyhow::{bail, Ok, Result};

use crate::ast::{Decl, Expr, Func, Ident, Lit, Module, Statement, Type, VarType};

pub struct Generator {
    pub writer: Writer,
    pub globals: HashSet<Ident>,
    pub types: HashMap<Ident, Type>,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            writer: Writer::new(),
            globals: HashSet::new(),
            types: HashMap::new(),
        }
    }

    pub fn set_globals(&mut self, globals: HashSet<Ident>) {
        self.globals = globals;
    }

    pub fn set_types(&mut self, types: HashMap<Ident, Type>) {
        self.types = types;
    }

    pub fn run(&mut self, module: &mut Module) -> Result<()> {
        self.module(module)?;

        Ok(())
    }

    fn module(&mut self, module: &mut Module) -> Result<()> {
        self.writer.start();
        self.writer.write("module");

        self.writer.start();
        self.writer.write(r#"import "env" "memory" (memory 1)"#);
        self.writer.end();

        for decl in &mut module.0 {
            self.decl(decl)?;
        }

        // builtin functions here
        self.writer.write(
            r#"
(func $_init (result i32)
    ;; stack pointer
    i32.const 0
    i32.const 1
    i32.store
    ;; call $main
    call $main
)
(func $alloc (param $size i32) (result i32)
    (local $addr i32)

    ;; get stack pointer
    i32.const 0
    i32.load
    local.set $addr

    i32.const 0
    ;; new pointer
    local.get $addr
    local.get $size
    i32.add
    ;; store new stack pointer
    i32.store

    ;; return old stack pointer
    local.get $addr
    local.get $size
    i32.sub
)
"#,
        );

        self.writer.start();
        self.writer.write(r#"export "main" (func $_init)"#);
        self.writer.end();
        self.writer.end();

        self.writer.finalize();

        Ok(())
    }

    fn decl(&mut self, decl: &mut Decl) -> Result<()> {
        match decl {
            Decl::Func(func) => self.func(func),
            Decl::Let(ident, type_, expr) => self.global_let(ident, type_, expr),
            Decl::Type(_, _) => Ok(()),
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
                let type_resolved = if let Type::Ident(ident) = type_ {
                    self.types.get(ident).unwrap().clone()
                } else {
                    type_.clone()
                };

                self.writer.start();
                self.writer.write(&format!(
                    "local ${} {}",
                    ident.as_str(),
                    TypeRep::from_type(&type_resolved)?.to_wasm_type()
                ));
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
            Expr::Record(_, fields) => {
                // allocate memory
                self.writer.new_statement();
                self.writer.write(&format!("i32.const {}", fields.len()));
                self.writer.new_statement();
                self.writer.write("call $alloc");

                for (i, (_, expr)) in fields.into_iter().enumerate() {
                    self.writer.new_statement();

                    self.writer.new_statement();
                    self.expr(expr)?;

                    self.writer.new_statement();
                    self.writer.write("local.tee $value");
                    self.writer.new_statement();
                    self.writer.write("local.get $address");
                    self.writer.new_statement();
                    self.writer.write(&format!("i32.const {}", i));
                    self.writer.new_statement();
                    self.writer.write("i32.add");
                    self.writer.new_statement();
                    self.writer.write("local.get $value");
                    self.writer.new_statement();
                    self.writer.write("i32.store");
                }

                Ok(())
            }
            Expr::Project(expr, type_, label) => {
                self.expr(expr)?;

                self.writer.new_statement();
                let fields = self.types[&type_.clone().to_ident()?].clone().to_record()?;
                self.writer.write(&format!(
                    "i32.const {}",
                    fields.iter().position(|(l, _)| l == label).unwrap()
                ));
                self.writer.new_statement();
                self.writer.write("i32.add");
                self.writer.new_statement();
                self.writer.write("i32.load");

                Ok(())
            }
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
            self.write(&format!("\n{}", " ".repeat(self.depth * 2)));
        }
        self.index = 0;
    }

    pub fn finalize(&mut self) {
        for _ in 0..self.depth {
            self.end();
        }
    }
}

enum TypeRep {
    I32,
    Pointer,
}

impl TypeRep {
    fn from_type(type_: &Type) -> Result<TypeRep> {
        match type_ {
            Type::Omit(_) => todo!(),
            Type::I32 => Ok(TypeRep::I32),
            Type::Nil => todo!(),
            Type::Bool => Ok(TypeRep::I32),
            Type::Func(_, _) => todo!(),
            Type::Record(_) => Ok(TypeRep::Pointer),
            Type::Ident(_) => todo!(),
        }
    }

    fn to_wasm_type(&self) -> String {
        match self {
            TypeRep::I32 => "i32",
            TypeRep::Pointer => "i32",
        }
        .to_string()
    }
}
