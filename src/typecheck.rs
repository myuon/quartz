use std::collections::HashMap;

use anyhow::{anyhow, bail, Context, Result};

use crate::{
    ast::{Decl, Expr, Func, Lit, Module, Statement, Type},
    compiler::ErrorInSource,
    util::{ident::Ident, path::Path, source::Source},
};

pub struct TypeChecker {
    omits: Constrains,
    locals: HashMap<Ident, Type>,
    pub globals: HashMap<Path, Type>,
    pub types: HashMap<Ident, Type>,
    current_path: Path,
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
                    "sub",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::I32)),
                ),
                (
                    "mult",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::I32)),
                ),
                (
                    "div",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::I32)),
                ),
                (
                    "mod",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::I32)),
                ),
                (
                    "equal",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::Bool)), // FIXME: support bool
                ),
                ("not", Type::Func(vec![Type::Bool], Box::new(Type::Bool))),
                (
                    "lt",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::Bool)),
                ),
                (
                    "gt",
                    Type::Func(vec![Type::I32, Type::I32], Box::new(Type::Bool)),
                ),
                (
                    "alloc",
                    Type::Func(vec![Type::I32], Box::new(Type::Ptr(Box::new(Type::I32)))),
                ),
                (
                    "write_stdout",
                    Type::Func(vec![Type::Byte], Box::new(Type::Nil)),
                ),
                (
                    "mem_copy",
                    Type::Func(
                        vec![
                            Type::Ptr(Box::new(Type::I32)),
                            Type::Ptr(Box::new(Type::I32)),
                            Type::I32,
                        ],
                        Box::new(Type::Nil),
                    ),
                ),
                (
                    "mem_free",
                    Type::Func(vec![Type::Ptr(Box::new(Type::I32))], Box::new(Type::Nil)),
                ),
                (
                    "debug_i32",
                    Type::Func(vec![Type::I32], Box::new(Type::Nil)),
                ),
            ]
            .into_iter()
            .map(|(k, v)| (Path::ident(Ident(k.to_string())), v))
            .collect(),
            types: vec![
                (
                    "string",
                    Type::Record(vec![
                        (Ident("data".to_string()), Type::Ptr(Box::new(Type::Byte))),
                        (Ident("length".to_string()), Type::I32),
                    ]),
                ),
                (
                    "array",
                    Type::Record(vec![
                        (Ident("data".to_string()), Type::Ptr(Box::new(Type::Byte))),
                        (Ident("length".to_string()), Type::I32),
                    ]),
                ),
                (
                    "vec",
                    Type::Record(vec![
                        (Ident("data".to_string()), Type::Ptr(Box::new(Type::I32))),
                        (Ident("length".to_string()), Type::I32),
                        (Ident("capacity".to_string()), Type::I32),
                    ]),
                ),
            ]
            .into_iter()
            .map(|(k, v)| (Ident(k.to_string()), v))
            .collect(),
            current_path: Path::empty(),
        }
    }

    pub fn run(&mut self, module: &mut Module) -> Result<()> {
        self.module(module)?;

        Ok(())
    }

    fn module(&mut self, module: &mut Module) -> Result<()> {
        self.module_register_for_back_reference(module)?;
        self.module_typecheck(module)?;

        Ok(())
    }

    fn module_register_for_back_reference(&mut self, module: &mut Module) -> Result<()> {
        for decl in &mut module.0 {
            match decl {
                Decl::Func(func) => {
                    self.globals
                        .insert(self.path_to(&func.name), func.to_type());
                }
                Decl::Let(ident, type_, _expr) => {
                    self.globals.insert(self.path_to(&ident), type_.clone());
                }
                Decl::Type(ident, type_) => {
                    self.types.insert(ident.clone(), type_.clone());
                }
                Decl::Module(name, module) => {
                    let path = self.current_path.clone();
                    self.current_path.push(name.clone());
                    self.module_register_for_back_reference(module)?;
                    self.current_path = path;
                }
            }
        }

        Ok(())
    }

    fn module_typecheck(&mut self, module: &mut Module) -> Result<()> {
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
                self.globals
                    .insert(self.path_to(&func.name), func.to_type());
            }
            Decl::Let(ident, type_, expr) => {
                let mut result = self.expr(expr)?;
                self.unify(type_, &mut result).context(ErrorInSource {
                    start: expr.start.unwrap_or(0),
                    end: expr.end.unwrap_or(0),
                })?;

                self.globals.insert(self.path_to(&ident), type_.clone());
            }
            Decl::Type(ident, type_) => {
                self.types.insert(ident.clone(), type_.clone());
            }
            Decl::Module(name, module) => {
                let path = self.current_path.clone();
                self.current_path.push(name.clone());
                self.module(module)?;
                self.current_path = path;
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

        if expected.is_omit() {
            self.unify(expected, &mut Type::Nil)?;
        }

        Ok(())
    }

    fn statement(&mut self, statement: &mut Statement) -> Result<Option<Type>> {
        match statement {
            Statement::Let(val, type_, expr) => {
                let mut result = self.expr(expr)?;
                self.unify(type_, &mut result).context(ErrorInSource {
                    start: expr.start.unwrap_or(0),
                    end: expr.end.unwrap_or(0),
                })?;

                self.locals.insert(val.clone(), type_.clone());

                Ok(None)
            }
            Statement::Return(expr) => Ok(Some(self.expr(expr)?)),
            Statement::Expr(expr) => {
                self.expr(expr)?;
                Ok(None)
            }
            Statement::Assign(lhs, rhs) => {
                let mut lhs_type = self.expr_left_value(lhs)?;
                let mut rhs_type = Type::Ptr(Box::new(self.expr(rhs)?));
                self.unify(&mut lhs_type, &mut rhs_type)
                    .context(ErrorInSource {
                        start: lhs.start.unwrap_or(0),
                        end: lhs.end.unwrap_or(0),
                    })?;

                Ok(None)
            }
            Statement::If(cond, type_, then_block, else_block) => {
                let mut cond_type = self.expr(cond)?;
                self.unify(&mut cond_type, &mut Type::Bool)
                    .context(ErrorInSource {
                        start: cond.start.unwrap_or(0),
                        end: cond.end.unwrap_or(0),
                    })?;

                let mut then_type = Type::Omit(0);
                self.block(then_block, &mut then_type)?;

                if let Some(else_block) = else_block {
                    self.block(else_block, &mut then_type)?;
                }

                self.unify(type_, &mut Type::Nil)?;

                Ok(Some(then_type))
            }
            Statement::While(cond, block) => {
                let mut cond_type = self.expr(cond)?;
                self.unify(&mut cond_type, &mut Type::Bool)
                    .context(ErrorInSource {
                        start: cond.start.unwrap_or(0),
                        end: cond.end.unwrap_or(0),
                    })?;

                let mut block_type = Type::Omit(0);
                self.block(block, &mut block_type)?;

                Ok(None)
            }
            Statement::For(ident, range, body) => {
                let type_ = self.expr(range)?;
                let element = type_.as_range_type()?;

                self.locals.insert(ident.clone(), element.clone());

                let mut body_type = Type::Omit(0);
                self.block(body, &mut body_type)?;

                Ok(None)
            }
        }
    }

    fn expr(&mut self, expr: &mut Source<Expr>) -> Result<Type> {
        match &mut expr.data {
            Expr::Lit(lit) => self.lit(lit),
            Expr::Ident(ident) => self.ident(ident),
            Expr::Path(path) => self
                .globals
                .get(path)
                .cloned()
                .ok_or(anyhow!("unknown path: {:?}", path)),
            Expr::Call(caller, args) => self.call(caller, args),
            Expr::Record(ident, record) => {
                let mut field_types = self
                    .resolve_record_type(Type::Ident(ident.clone()))
                    .context(ErrorInSource {
                        start: expr.start.unwrap_or(0),
                        end: expr.end.unwrap_or(0),
                    })?
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
                    )
                    .context(ErrorInSource {
                        start: expr.start.unwrap_or(0),
                        end: expr.end.unwrap_or(0),
                    })?;
                }

                Ok(Type::Ident(ident.clone()))
            }
            Expr::Project(expr, type_, label) => {
                let mut expr_type = self.expr(expr)?;
                self.unify(type_, &mut expr_type).context(ErrorInSource {
                    start: expr.start.unwrap_or(0),
                    end: expr.end.unwrap_or(0),
                })?;

                // methods for builtin types
                if let Type::Ptr(p) = &mut expr_type {
                    match label.as_str() {
                        "at" => {
                            return Ok(Type::Func(vec![Type::I32], p.clone()));
                        }
                        _ => (),
                    }
                }
                if let Type::Array(p, _) = &mut expr_type {
                    match label.as_str() {
                        "at" => {
                            return Ok(Type::Func(vec![Type::I32], p.clone()));
                        }
                        _ => (),
                    }
                }
                if let Type::Vec(p) = &mut expr_type {
                    match label.as_str() {
                        "at" => {
                            return Ok(Type::Func(vec![Type::I32], p.clone()));
                        }
                        "push" => {
                            return Ok(Type::Func(vec![p.as_ref().clone()], Box::new(Type::Nil)));
                        }
                        _ => (),
                    }
                }

                let fields = self
                    .resolve_record_type(expr_type.clone())
                    .context(ErrorInSource {
                        start: expr.start.unwrap_or(0),
                        end: expr.end.unwrap_or(0),
                    })?
                    .into_iter()
                    .collect::<HashMap<_, _>>();

                if fields.contains_key(label) {
                    // field access

                    Ok(fields[label].clone())
                } else {
                    // method access
                    let path = Path::new(vec![expr_type.clone().to_ident()?, label.clone()]);

                    let type_ = self
                        .globals
                        .get(&path)
                        .ok_or(anyhow!("unknown method: {:?}", path))?;
                    let (mut arg_types, result_type) = type_.clone().to_func()?;
                    self.unify(&mut expr_type, &mut arg_types[0])?;

                    Ok(Type::Func(arg_types[1..].to_vec(), result_type))
                }
            }
            Expr::Make(type_, args) => {
                assert_eq!(args, &mut vec![]);

                Ok(type_.clone())
            }
            Expr::Range(start, end) => {
                let mut start_type = self.expr(start)?;
                let mut end_type = self.expr(end)?;

                self.unify(&mut start_type, &mut end_type)
                    .context(ErrorInSource {
                        start: start.start.unwrap_or(0),
                        end: start.end.unwrap_or(0),
                    })?;

                Ok(Type::Range(Box::new(start_type)))
            }
            Expr::As(expr, type_) => {
                self.expr(expr)?;

                Ok(type_.clone())
            }
            Expr::SizeOf(_) => Ok(Type::I32),
            Expr::Self_ => {
                assert_eq!(self.current_path.0.len(), 1);
                let ident = self.current_path.0[0].clone();

                Ok(Type::Ident(ident))
            }
            Expr::Equal(lhs, rhs) => {
                let mut lhs_type = self.expr(lhs)?;
                let mut rhs_type = self.expr(rhs)?;

                self.unify(&mut lhs_type, &mut rhs_type)
                    .context(ErrorInSource {
                        start: lhs.start.unwrap_or(0),
                        end: lhs.end.unwrap_or(0),
                    })?;

                Ok(Type::Bool)
            }
            Expr::NotEqual(lhs, rhs) => {
                let mut lhs_type = self.expr(lhs)?;
                let mut rhs_type = self.expr(rhs)?;

                self.unify(&mut lhs_type, &mut rhs_type)
                    .context(ErrorInSource {
                        start: lhs.start.unwrap_or(0),
                        end: lhs.end.unwrap_or(0),
                    })?;

                Ok(Type::Bool)
            }
            Expr::Wrap(expr) => {
                let expr_type = self.expr(expr)?;

                Ok(Type::Optional(Box::new(expr_type)))
            }
        }
    }

    fn expr_left_value(&mut self, expr: &mut Source<Expr>) -> Result<Type> {
        Ok(Type::Ptr(Box::new(self.expr(expr)?)))
    }

    fn lit(&mut self, lit: &mut Lit) -> Result<Type> {
        match lit {
            Lit::Nil => Ok(Type::Nil),
            Lit::Bool(_) => Ok(Type::Bool),
            Lit::I32(_) => Ok(Type::I32),
            Lit::String(_) => Ok(Type::Ident(Ident("string".to_string()))),
        }
    }

    fn ident_local(&mut self, ident: &mut Ident) -> Result<Type> {
        match self.locals.get(ident) {
            Some(type_) => Ok(type_.clone()),
            None => bail!("Ident Not Found: {}", ident.as_str()),
        }
    }

    fn ident_global(&mut self, ident: &mut Ident) -> Result<Type> {
        match self.globals.get(&Path::ident(ident.clone())) {
            Some(type_) => Ok(type_.clone()),
            None => bail!("Ident Not Found: {}", ident.as_str()),
        }
    }

    fn ident(&mut self, ident: &mut Ident) -> Result<Type> {
        self.ident_local(ident).or(self.ident_global(ident))
    }

    fn call(
        &mut self,
        caller: &mut Box<Source<Expr>>,
        args: &mut Vec<Source<Expr>>,
    ) -> Result<Type> {
        let (mut arg_types, result_type) = self.expr(caller.as_mut())?.to_func()?;
        if arg_types.len() != args.len() {
            return Err(anyhow!(
                "wrong number of arguments, expected {}, but found {}",
                arg_types.len(),
                args.len()
            )
            .context(ErrorInSource {
                start: caller.start.unwrap_or(0),
                end: caller.end.unwrap_or(0),
            }));
        }

        for (index, arg) in args.into_iter().enumerate() {
            let mut arg_type = self.expr(arg)?;
            self.unify(&mut arg_types[index], &mut arg_type)
                .context(ErrorInSource {
                    start: arg.start.unwrap_or(0),
                    end: arg.end.unwrap_or(0),
                })?
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
            Type::Array(_, _) => self.resolve_record_type(Type::Ident(Ident("array".to_string()))),
            Type::Vec(_) => self.resolve_record_type(Type::Ident(Ident("vec".to_string()))),
            _ => bail!("expected record type, but found {:?}", type_),
        }
    }

    fn path_to(&self, ident: &Ident) -> Path {
        let mut path = self.current_path.clone();
        path.push(ident.clone());

        path
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

    pub fn unify(type1: &mut Type, type2: &mut Type) -> Result<Constrains> {
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

                constrains.merge(&Constrains::unify(ret1.as_mut(), ret2.as_mut())?);

                Ok(constrains)
            }
            (Type::Ptr(type1), Type::Ptr(type2)) => {
                Constrains::unify(type1.as_mut(), type2.as_mut())
            }
            (Type::Ident(ident), Type::Vec(_)) if ident.as_str() == "vec" => {
                Ok(Constrains::empty())
            }
            (Type::Vec(_), Type::Ident(ident)) if ident.as_str() == "vec" => {
                Ok(Constrains::empty())
            }
            (Type::Nil, Type::Optional(_)) => Ok(Constrains::empty()),
            (Type::Optional(_), Type::Nil) => Ok(Constrains::empty()),
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
