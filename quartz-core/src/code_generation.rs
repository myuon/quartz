use std::collections::HashMap;

use anyhow::Result;

use crate::{
    ast::{Declaration, Expr, Function, Module, Statement},
    vm::QVMInstruction,
};

#[derive(Debug)]
pub struct CodeGeneration {
    code: Vec<QVMInstruction>,
    global_pointer: usize,
    globals: HashMap<String, usize>,
    arg_pointer: usize,
    args: HashMap<String, usize>,
    local_pointer: usize,
    locals: HashMap<String, usize>,
}

impl CodeGeneration {
    pub fn new() -> CodeGeneration {
        CodeGeneration {
            code: vec![],
            global_pointer: 0,
            globals: HashMap::new(),
            arg_pointer: 0,
            args: HashMap::new(),
            local_pointer: 0,
            locals: HashMap::new(),
        }
    }

    fn push_local(&mut self, name: String) {
        self.locals.insert(name, self.local_pointer);
        self.local_pointer += 1;
    }

    fn push_arg(&mut self, name: String) {
        self.args.insert(name, self.arg_pointer);
        self.arg_pointer += 1;
    }

    fn push_global(&mut self, name: String) {
        self.globals.insert(name, self.global_pointer);
        self.global_pointer += 1;
    }

    fn expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Var(v) => {
                if let Some(u) = self.locals.get(v) {
                    self.code.push(QVMInstruction::Load(*u));
                } else if let Some(u) = self.args.get(v) {
                    self.code.push(QVMInstruction::LoadArg(*u));
                } else if let Some(u) = self.globals.get(v) {
                    self.code.push(QVMInstruction::GlobalGet(*u));
                } else {
                    anyhow::bail!("{} not found", v);
                }
            }
            Expr::Lit(lit) => {
                use crate::ast::Literal::*;

                match lit {
                    Nil => {
                        self.code.push(QVMInstruction::I32Const(9999));
                    }
                    Bool(_) => todo!(),
                    Int(n) => {
                        self.code.push(QVMInstruction::I32Const(*n));
                    }
                    String(_) => todo!(),
                }
            }
            Expr::Call(f, es) => {
                for e in es {
                    self.expr(e)?;
                }

                if let Expr::Var(v) = f.as_ref() {
                    // builtin functions
                    if v == "_add" {
                        self.code.push(QVMInstruction::Add);
                    } else {
                        self.code.push(QVMInstruction::PlaceholderLabel(v.clone()));
                    }
                } else {
                    todo!();
                }
            }
            Expr::Struct(_, _) => todo!(),
            Expr::Project(_, _, _, _) => todo!(),
            Expr::Deref(_) => todo!(),
            Expr::Ref(_) => todo!(),
        }

        Ok(())
    }

    fn statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Let(v, expr) => {
                self.expr(expr)?;
                self.push_local(v.clone());
            }
            Statement::Expr(expr) => {
                self.expr(expr)?;
            }
            Statement::Return(e) => {
                self.expr(e)?;
                self.code.push(QVMInstruction::Return);
            }
            Statement::If(_, _, _) => todo!(),
            Statement::Continue => todo!(),
            Statement::Assignment(v, e) => {
                self.expr(e)?;

                match v.as_ref() {
                    Expr::Var(v) => {
                        if let Some(u) = self.locals.get(v).cloned() {
                            self.code.push(QVMInstruction::Store(u));
                        } else if let Some(u) = self.globals.get(v).cloned() {
                            self.code.push(QVMInstruction::GlobalSet(u));
                        } else {
                            anyhow::bail!("{} not found", v);
                        }
                    }
                    _ => todo!(),
                }
            }
            Statement::Loop(_) => todo!(),
            Statement::While(_, _) => todo!(),
        }

        Ok(())
    }

    fn function(&mut self, function: &Function) -> Result<()> {
        self.local_pointer = 0;
        self.locals.clear();

        for (name, _) in &function.args {
            self.push_arg(name.clone());
        }

        for b in &function.body {
            self.statement(b)?;
        }

        Ok(())
    }

    fn variable(&mut self, name: &String, expr: &Expr) -> Result<()> {
        self.expr(expr)?;
        self.push_global(name.clone());
        self.code
            .push(QVMInstruction::GlobalSet(self.globals[name]));

        Ok(())
    }

    pub fn globals(&self) -> usize {
        self.globals.len()
    }

    pub fn generate(&mut self, module: &Module) -> Result<Vec<QVMInstruction>> {
        let mut init_code = vec![];
        let mut function_code = HashMap::new();

        // first path
        for decl in &module.0 {
            self.code = vec![];

            match decl {
                Declaration::Function(f) => {
                    self.function(f)?;
                    function_code.insert(f.name.clone(), self.code.clone());
                }
                Declaration::Variable(name, expr) => {
                    self.variable(name, expr)?;
                    init_code.extend(self.code.clone());
                }
                Declaration::Struct(_) => {}
            };
        }

        // call main
        init_code.extend(vec![
            QVMInstruction::I32Const(999), // for return value
            QVMInstruction::PlaceholderLabel("main".to_string()),
            QVMInstruction::Return,
        ]);

        // second path
        let mut labels = HashMap::new();
        let mut code = vec![];
        code.extend(init_code);
        for (k, v) in function_code {
            labels.insert(k, code.len());
            code.extend(v);
        }

        // resolve labels
        for i in 0..code.len() {
            if let QVMInstruction::PlaceholderLabel(ref label) = code[i] {
                if let Some(pc) = labels.get(label) {
                    code[i] = QVMInstruction::Call(*pc);
                } else {
                    anyhow::bail!("label {} not found", label);
                }
            }
        }

        Ok(code)
    }
}

pub fn generate(module: &Module) -> Result<Vec<QVMInstruction>> {
    let mut g = CodeGeneration::new();
    let code = g.generate(module)?;

    Ok(code)
}
