use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::ast::{Decl, Expr, Func, Ident, Lit, Module, Statement, Type, VarType};

pub struct TypeChecker {
    omits: Constrains,
    locals: HashMap<Ident, Type>,
    pub globals: HashMap<Ident, Type>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            omits: Constrains::empty(),
            locals: HashMap::new(),
            globals: vec![(
                "add",
                Type::Func(vec![Type::I32, Type::I32], Box::new(Type::I32)),
            )]
            .into_iter()
            .map(|(k, v)| (Ident(k.to_string()), v))
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
            Decl::Func(func) => {
                self.func(func)?;
                self.globals.insert(func.name.clone(), func.to_type());
            }
            Decl::Let(ident, type_, expr) => {
                let mut result = self.expr(expr)?;
                self.unify(type_, &mut result)?;

                self.globals.insert(ident.clone(), type_.clone());
            }
        }

        Ok(())
    }

    fn func(&mut self, func: &mut Func) -> Result<()> {
        let locals = self.locals.clone();
        for (name, type_) in &mut func.params {
            self.locals.insert(name.clone(), type_.clone());
        }

        for statement in &mut func.body {
            if let Some(result) = &mut self.statement(statement)? {
                self.unify(&mut func.result, result)?;
            }
        }

        self.locals = locals;

        if func.result.is_omit() {
            func.result = Type::Nil;
        }

        Ok(())
    }

    fn statement(&mut self, statement: &mut Statement) -> Result<Option<Type>> {
        match statement {
            Statement::Let(val, type_, expr) => {
                let mut result = self.expr(expr)?;
                self.unify(type_, &mut result)?;

                self.locals.insert(val.clone(), type_.clone());

                Ok(None)
            }
            Statement::Return(expr) => Ok(Some(self.expr(expr)?)),
            Statement::Expr(expr) => {
                self.expr(expr)?;
                Ok(None)
            }
            Statement::Assign(var_type, lhs, rhs) => {
                let mut lhs_type = self.ident(lhs)?;
                let mut rhs_type = self.expr(rhs)?;
                self.unify(&mut lhs_type, &mut rhs_type)?;

                *var_type = Some(if self.ident_local(lhs).is_ok() {
                    VarType::Local
                } else if self.ident_global(lhs).is_ok() {
                    VarType::Global
                } else {
                    bail!("unknown variable: {}", lhs.as_str());
                });

                Ok(None)
            }
        }
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

    fn ident_local(&mut self, ident: &mut Ident) -> Result<Type> {
        match self.locals.get(ident) {
            Some(type_) => Ok(type_.clone()),
            None => bail!("Ident Not Found: {}", ident.as_str()),
        }
    }

    fn ident_global(&mut self, ident: &mut Ident) -> Result<Type> {
        match self.globals.get(ident) {
            Some(type_) => Ok(type_.clone()),
            None => bail!("Ident Not Found: {}", ident.as_str()),
        }
    }

    fn ident(&mut self, ident: &mut Ident) -> Result<Type> {
        self.ident_local(ident).or(self.ident_global(ident))
    }

    fn call(&mut self, caller: &mut Ident, args: &mut Vec<Expr>) -> Result<Type> {
        let (mut arg_types, result_type) = self.ident(caller)?.to_func()?;
        if arg_types.len() != args.len() {
            bail!(
                "wrong number of arguments, expected {}, but found {}",
                arg_types.len(),
                args.len()
            );
        }

        for (index, arg) in args.into_iter().enumerate() {
            let mut arg_type = self.expr(arg)?;
            self.unify(&mut arg_types[index], &mut arg_type)?
        }

        Ok(result_type.as_ref().clone())
    }

    fn unify(&mut self, type1: &mut Type, type2: &mut Type) -> Result<()> {
        let cs = Constrains::unify(type1, type2)?;
        self.omits.merge(&cs);

        cs.apply(type1);
        cs.apply(type2);

        Ok(())
    }
}

struct Constrains {
    constrains: HashMap<usize, Type>,
}

impl Constrains {
    pub fn empty() -> Constrains {
        Constrains {
            constrains: HashMap::new(),
        }
    }

    pub fn unify(type1: &Type, type2: &Type) -> Result<Constrains> {
        match (type1, type2) {
            (type1, type2) if type1 == type2 => Ok(Constrains::empty()),
            (Type::Omit(i), type_) => {
                let mut constrains = Constrains::empty();
                constrains.constrains.insert(*i, type_.clone());
                Ok(constrains)
            }
            (type_, Type::Omit(i)) => {
                let mut constrains = Constrains::empty();
                constrains.constrains.insert(*i, type_.clone());
                Ok(constrains)
            }
            (Type::I32, Type::I32) => Ok(Constrains::empty()),
            (Type::Func(args1, ret1), Type::Func(args2, ret2)) => {
                if args1.len() != args2.len() {
                    bail!(
                        "wrong number of arguments, expected {}, but found {}",
                        args1.len(),
                        args2.len()
                    );
                }

                let mut constrains = Constrains::empty();
                for (arg1, arg2) in args1.into_iter().zip(args2.into_iter()) {
                    constrains.merge(&Constrains::unify(arg1, arg2)?);
                }

                constrains.merge(&Constrains::unify(ret1.as_ref(), ret2.as_ref())?);

                Ok(constrains)
            }
            (type1, type2) => {
                bail!(
                    "type mismatch, expected {}, but found {}",
                    type1.to_string(),
                    type2.to_string()
                );
            }
        }
    }

    fn merge(&mut self, other: &Constrains) {
        for (i, type_) in other.constrains.iter() {
            self.constrains.insert(*i, type_.clone());
        }
    }

    fn apply(&self, type_: &mut Type) {
        match type_ {
            Type::Omit(i) => {
                if let Some(result) = self.constrains.get(i) {
                    *type_ = result.clone();
                }
            }
            Type::Func(args, ret) => {
                for arg in args {
                    self.apply(arg);
                }
                self.apply(ret.as_mut());
            }
            _ => {}
        }
    }
}
