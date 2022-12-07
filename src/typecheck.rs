use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};

use crate::ast::{Decl, Expr, Func, Ident, Lit, Module, Statement, Type};

pub struct TypeChecker {
    omits: Constrains,
    locals: HashMap<Ident, Type>,
    pub globals: HashMap<Ident, Type>,
    pub types: HashMap<Ident, Type>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            omits: Constrains::empty(),
            locals: HashMap::new(),
            globals: vec![
                (
                    "add",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::I32)),
                ),
                (
                    "mult",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::I32)),
                ),
                (
                    "sub",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::I32)),
                ),
                (
                    "equal",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::Bool)), // FIXME: support bool
                ),
                (
                    "lt",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::Bool)),
                ),
            ]
            .into_iter()
            .map(|(k, v)| (Ident(k.to_string()), v))
            .collect(),
            types: HashMap::new(),
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
            Decl::Type(ident, type_) => {
                self.types.insert(ident.clone(), type_.clone());
            }
        }

        Ok(())
    }

    fn func(&mut self, func: &mut Func) -> Result<()> {
        let locals = self.locals.clone();

        self.locals.insert(func.name.clone(), func.to_type());

        for (name, type_) in &mut func.params {
            self.locals.insert(name.clone(), type_.clone());
        }

        self.block(&mut func.body, &mut func.result)?;
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

    fn block(&mut self, statements: &mut Vec<Statement>, expected: &mut Type) -> Result<()> {
        for statement in statements {
            if let Some(result) = &mut self.statement(statement)? {
                self.unify(expected, result)?;
            }
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
            Statement::Assign(lhs, rhs) => {
                let mut lhs_type = self.expr(lhs)?;
                let mut rhs_type = self.expr(rhs)?;
                self.unify(&mut lhs_type, &mut rhs_type)?;

                Ok(None)
            }
            Statement::If(cond, type_, then_block, else_block) => {
                let mut cond_type = self.expr(cond)?;
                self.unify(&mut cond_type, &mut Type::Bool)?;

                let mut then_type = Type::Omit(0);
                self.block(then_block, &mut then_type)?;

                if let Some(else_block) = else_block {
                    self.block(else_block, &mut then_type)?;
                }

                self.unify(type_, &mut then_type)?;

                Ok(None)
            }
            Statement::While(cond, block) => {
                let mut cond_type = self.expr(cond)?;
                self.unify(&mut cond_type, &mut Type::Bool)?;

                let mut block_type = Type::Omit(0);
                self.block(block, &mut block_type)?;

                Ok(None)
            }
        }
    }

    fn expr(&mut self, expr: &mut Expr) -> Result<Type> {
        match expr {
            Expr::Lit(lit) => self.lit(lit),
            Expr::Ident(ident) => self.ident(ident),
            Expr::Call(caller, args) => self.call(caller, args),
            Expr::Record(ident, record) => {
                let mut field_types = self
                    .resolve_record_type(Type::Ident(ident.clone()))?
                    .into_iter()
                    .collect::<HashMap<_, _>>();

                if field_types.len() != record.len() {
                    bail!("invalid number of fields");
                }

                for (field, expr) in record {
                    let mut expr_type = self.expr(expr)?;
                    self.unify(
                        &mut expr_type,
                        field_types
                            .get_mut(field)
                            .ok_or(anyhow!("unknown field: {}", field.as_str()))?,
                    )?;
                }

                Ok(Type::Ident(ident.clone()))
            }
            Expr::Project(expr, type_, label) => {
                let mut expr_type = self.expr(expr)?;
                self.unify(type_, &mut expr_type)?;

                let fields = self.resolve_record_type(expr_type.clone())?;
                let type_ = fields
                    .into_iter()
                    .find(|p| p.0 .0 == label.0)
                    .ok_or(anyhow!(
                        "field `{:?}` not found in record `{:?}`",
                        label,
                        expr_type
                    ))?
                    .1;

                Ok(type_.clone())
            }
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

    fn resolve_record_type(&mut self, type_: Type) -> Result<Vec<(Ident, Type)>> {
        match type_ {
            Type::Ident(ident) => {
                let type_ = self
                    .types
                    .get(&ident)
                    .ok_or(anyhow!("unknown type: {}", ident.as_str()))?;
                let fields = type_.clone().to_record()?;

                Ok(fields)
            }
            Type::Record(fields) => Ok(fields),
            _ => bail!("expected record type, but found {:?}", type_),
        }
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
