use anyhow::Result;

use crate::{
    ast::{Declaration, Expr, Function, Module, Statement},
    vm::QVMInstruction,
};

pub struct CodeGeneration {
    globals: usize,
    code: Vec<QVMInstruction>,
}

impl CodeGeneration {
    pub fn new() -> CodeGeneration {
        CodeGeneration {
            globals: 0,
            code: Vec::new(),
        }
    }

    fn get_pc(&self) -> usize {
        self.code.len()
    }

    fn get_global_count(&self) -> usize {
        self.globals
    }

    fn expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Var(_) => todo!(),
            Expr::Lit(_) => todo!(),
            Expr::Call(_, _) => todo!(),
            Expr::Struct(_, _) => todo!(),
            Expr::Project(_, _, _, _) => todo!(),
            Expr::Deref(_) => todo!(),
            Expr::Ref(_) => todo!(),
        }
    }

    fn statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Let(_, _) => todo!(),
            Statement::Expr(_) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::If(_, _, _) => todo!(),
            Statement::Continue => todo!(),
            Statement::Assignment(_, _) => todo!(),
            Statement::Loop(_) => todo!(),
            Statement::While(_, _) => todo!(),
        }
    }

    fn function(&mut self, function: &Function) -> Result<()> {
        let pc = self.get_pc();

        for b in &function.body {
            self.statement(b)?;
        }

        Ok(())
    }

    fn variable(&mut self, name: &String, expr: &Expr) -> Result<()> {
        Ok(())
    }

    fn decl(&mut self, decl: &Declaration) -> Result<()> {
        match decl {
            Declaration::Function(f) => self.function(f),
            Declaration::Variable(name, expr) => self.variable(name, expr),
            Declaration::Struct(s) => Ok(()),
        }
    }

    fn module(&mut self, module: &Module) -> Result<()> {
        for decl in &module.0 {
            self.decl(decl)?;
        }

        Ok(())
    }
}

pub fn generate(module: &Module) -> Result<Vec<QVMInstruction>> {
    let mut g = CodeGeneration::new();
    g.module(module)?;

    Ok(g.code)
}
