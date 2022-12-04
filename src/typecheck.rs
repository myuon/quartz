use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::ast::{Decl, Expr, Func, Ident, Lit, Module, Statement, Type};

pub struct TypeChecker {
    locals: HashMap<String, Type>,
    globals: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            locals: HashMap::new(),
            globals: vec![(
                "add",
                Type::Func(vec![Type::I32, Type::I32], Box::new(Type::I32)),
            )]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        }
    }

    pub fn run(&mut self, module: &mut Module) -> Result<()> {
        self.module(module)?;

        Ok(())
    }

    fn module(&mut self, module: &mut Module) -> Result<()> {
        for decl in &mut module.0 {
            self.locals.clear();
            self.decl(decl)?;
        }

        Ok(())
    }

    fn decl(&mut self, decl: &mut Decl) -> Result<()> {
        match decl {
            Decl::Func(func) => self.func(func),
        }
    }

    fn func(&mut self, func: &mut Func) -> Result<()> {
        for statement in &mut func.body {
            self.statement(statement)?;
        }

        Ok(())
    }

    fn statement(&mut self, statement: &mut Statement) -> Result<()> {
        match statement {
            Statement::Let(val, type_, expr) => {
                self.expr(expr)?;
                self.locals.insert(val.as_str().to_string(), type_.clone());
            }
            Statement::Return(expr) => {
                self.expr(expr)?;
            }
        }

        Ok(())
    }

    fn expr(&mut self, expr: &mut Expr) -> Result<Type> {
        match expr {
            Expr::Lit(lit) => self.lit(lit),
            Expr::Ident(ident) => self.ident(ident),
            Expr::Call(caller, args) => self.call(caller, args),
        }
    }

    fn lit(&mut self, lit: &mut Lit) -> Result<Type> {
        match lit {
            Lit::I32(_) => Ok(Type::I32),
        }
    }

    fn ident(&mut self, ident: &mut Ident) -> Result<Type> {
        match (self.locals.get(ident.as_str())).or(self.globals.get(ident.as_str())) {
            Some(type_) => Ok(type_.clone()),
            None => bail!("Ident Not Found: {}", ident.as_str()),
        }
    }

    fn call(&mut self, caller: &mut Expr, args: &mut Vec<Expr>) -> Result<Type> {
        let (arg_types, result_type) = self.expr(caller)?.to_func()?;
        if arg_types.len() != args.len() {
            bail!(
                "wrong number of arguments, expected {}, but found {}",
                arg_types.len(),
                args.len()
            );
        }

        for (index, arg) in args.into_iter().enumerate() {
            if arg_types[index] != self.expr(arg)? {
                bail!(
                    "wrong type of argument, expected {}, but found {}",
                    arg_types[index].to_string(),
                    self.expr(arg)?.to_string()
                );
            }
        }

        Ok(result_type.as_ref().clone())
    }
}
