use std::collections::{HashMap, HashSet};

use anyhow::{bail, Ok, Result};

use crate::{
    ast::{Ident, Type},
    ir::{IrTerm, IrType},
    util::sexpr_writer::SExprWriter,
};

pub struct Generator {
    pub writer: SExprWriter,
    pub globals: HashSet<Ident>,
    pub types: HashMap<Ident, Type>,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            writer: SExprWriter::new(),
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

    pub fn run(&mut self, module: &mut IrTerm) -> Result<()> {
        match module {
            IrTerm::Module { elements } => {
                self.module(elements)?;
            }
            _ => bail!("Expected module"),
        }

        Ok(())
    }

    fn module(&mut self, elements: &mut Vec<IrTerm>) -> Result<()> {
        self.writer.start();
        self.writer.write("module");

        self.writer.start();
        self.writer.write(r#"import "env" "memory" (memory 1)"#);
        self.writer.end();

        for term in elements {
            self.decl(term)?;
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

    fn decl(&mut self, decl: &mut IrTerm) -> Result<()> {
        match decl {
            IrTerm::Func {
                name,
                params,
                result,
                body,
            } => self.func(name, params, result, body),
            IrTerm::GlobalLet { name, type_, value } => self.global_let(name, type_, value),
            _ => bail!("Expected func or global let, got {:?}", decl),
        }
    }

    fn func(
        &mut self,
        name: &mut String,
        params: &mut Vec<(String, IrType)>,
        result: &mut IrType,
        body: &mut Vec<IrTerm>,
    ) -> Result<()> {
        self.writer.start();
        self.writer.write("func");
        self.writer.write(&format!("${}", name.as_str()));
        for (name, type_) in params {
            self.writer
                .write(&format!("(param ${} {})", name.as_str(), type_.to_string()));
        }

        if !result.is_nil() {
            self.writer
                .write(&format!("(result {})", result.to_string()));
        }

        for term in body.iter() {
            for (name, type_, _) in term.find_let() {
                self.writer.start();
                self.writer
                    .write(&format!("local ${} {}", name.as_str(), type_.to_string()));
                self.writer.end();
            }
        }
        for expr in body.iter_mut() {
            self.expr(expr)?;
        }

        self.writer.end();

        Ok(())
    }

    fn global_let(
        &mut self,
        name: &mut String,
        type_: &mut IrType,
        expr: &mut IrTerm,
    ) -> Result<()> {
        self.writer.start();
        self.writer.write("global");
        self.writer.write(&format!("${}", name.as_str()));
        self.writer.write(&format!("(mut {})", type_.to_string()));
        self.writer.start();
        self.expr(expr)?;
        self.writer.end();
        self.writer.end();

        Ok(())
    }

    fn expr(&mut self, expr: &mut IrTerm) -> Result<()> {
        match expr {
            IrTerm::Nil => {
                self.writer.write(&format!("i32.const {}", -1));
            }
            IrTerm::I32(i) => {
                self.writer.write(&format!("i32.const {}", i));
            }
            IrTerm::Ident(i) => {
                if self.globals.contains(&Ident(i.clone())) {
                    self.writer.write(&format!("global.get ${}", i.as_str()));
                } else {
                    self.writer.write(&format!("local.get ${}", i.as_str()));
                }
            }
            IrTerm::Call { name, args } => self.call(name, args)?,
            IrTerm::Seq { elements } => {
                for element in elements {
                    self.expr(element)?;
                }
            }
            IrTerm::Let {
                name,
                type_: _,
                value,
            } => {
                self.writer.new_statement();
                self.expr(value)?;

                self.writer.new_statement();
                self.writer.write("local.set");
                self.expr_left_value(&mut IrTerm::Ident(name.clone()))?;
            }
            IrTerm::Return { value } => {
                self.writer.new_statement();
                self.expr(value)?;
            }
            IrTerm::AssignLocal { lhs, rhs } => {
                self.writer.new_statement();
                self.expr(rhs)?;

                self.writer.new_statement();
                self.writer.write("local.set");
                self.expr_left_value(lhs)?;
            }
            IrTerm::AssignGlobal { lhs, rhs } => {
                self.writer.new_statement();
                self.expr(rhs)?;

                self.writer.new_statement();
                self.writer.write("global.set");
                self.expr_left_value(lhs)?;
            }
            IrTerm::If {
                cond,
                type_,
                then,
                else_,
            } => {
                self.writer.new_statement();
                self.expr(cond)?;

                self.writer.write("if");
                if !type_.is_nil() {
                    self.writer
                        .write(&format!("(result {})", type_.to_string()));
                }

                self.expr(then)?;
                self.writer.write("else");
                self.expr(else_)?;

                self.writer.new_statement();
                self.writer.write("end");
            }
            IrTerm::GetField { address, offset } => {
                self.writer.new_statement();
                self.expr(address)?;

                self.writer.new_statement();
                self.expr(&mut IrTerm::I32(offset.clone() as i32))?;

                self.writer.new_statement();
                self.writer.write("i32.add");

                self.writer.new_statement();
                self.writer.write("i32.load");
            }
            IrTerm::SetField {
                address,
                offset,
                value,
            } => {
                self.writer.new_statement();
                self.expr(address)?;

                self.writer.new_statement();
                self.expr(&mut IrTerm::I32(offset.clone() as i32))?;

                self.writer.new_statement();
                self.writer.write("i32.add");

                self.writer.new_statement();
                self.expr(value)?;

                self.writer.new_statement();
                self.writer.write("i32.store");
            }
            _ => todo!(),
        }

        Ok(())
    }

    fn expr_left_value(&mut self, expr: &mut IrTerm) -> Result<()> {
        match expr {
            IrTerm::Ident(i) => {
                self.writer.write(&format!("${}", i.as_str()));
            }
            _ => todo!(),
        }

        Ok(())
    }

    fn call(&mut self, caller: &mut String, args: &mut Vec<IrTerm>) -> Result<()> {
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
