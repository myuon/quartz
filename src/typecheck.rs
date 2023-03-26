use std::collections::HashMap;

use anyhow::{anyhow, bail, Context, Result};

use crate::{
    ast::{
        Decl, Expr, ForMode, Func, Lit, Module, Pattern, Statement, Type, UnwrapMode, VariadicCall,
    },
    compiler::ErrorInSource,
    util::{ident::Ident, path::Path, source::Source},
};

pub struct TypeChecker {
    locals: HashMap<Ident, Source<Type>>,
    pub globals: HashMap<Path, Source<Type>>,
    pub types: HashMap<Ident, (Vec<Type>, Type)>,
    current_path: Path,
    imported: Vec<Path>,
    result_type: Option<Type>,
    pub search_node: Option<(Path, usize)>,
    pub search_node_type: Option<Type>,
    pub search_node_definition: Option<(Path, usize, usize)>,
    pub completion: Option<Vec<(String, String, String)>>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            locals: HashMap::new(),
            globals: vec![
                ("not", Type::Func(vec![Type::Bool], Box::new(Type::Bool))),
                (
                    "write_stdout",
                    Type::Func(vec![Type::Byte], Box::new(Type::Nil)),
                ),
                ("read_stdin", Type::Func(vec![], Box::new(Type::Byte))),
                (
                    "debug_i32",
                    Type::Func(vec![Type::I32], Box::new(Type::Nil)),
                ),
                ("debug", Type::Func(vec![Type::Any], Box::new(Type::Nil))),
                (
                    "xor_i64",
                    Type::Func(vec![Type::I64, Type::I64], Box::new(Type::I64)),
                ),
                (
                    "xor_u32",
                    Type::Func(vec![Type::U32, Type::U32], Box::new(Type::U32)),
                ),
                (
                    "i32_to_i64",
                    Type::Func(vec![Type::I32], Box::new(Type::I64)),
                ),
                (
                    "i64_to_i32",
                    Type::Func(vec![Type::I64], Box::new(Type::I32)),
                ),
                ("abort", Type::Func(vec![], Box::new(Type::Nil))),
                (
                    "reflection_get_type_rep_id",
                    Type::Func(vec![Type::Any], Box::new(Type::I32)),
                ),
                (
                    "reflection_is_pointer",
                    Type::Func(vec![Type::Any], Box::new(Type::Bool)),
                ),
                (
                    "reflection_is_bool",
                    Type::Func(vec![Type::Any], Box::new(Type::Bool)),
                ),
                (
                    "unsafe_load_ptr",
                    Type::Func(vec![Type::Any, Type::I32], Box::new(Type::Any)),
                ),
            ]
            .into_iter()
            .map(|(k, v)| (Path::ident(Ident(k.to_string())), Source::unknown(v)))
            .collect(),
            types: vec![
                (
                    "array",
                    vec![],
                    Type::Record(vec![
                        (Ident("data".to_string()), Type::Ptr(Box::new(Type::Byte))),
                        (Ident("length".to_string()), Type::I32),
                    ]),
                ),
                (
                    "vec",
                    vec![Type::Omit(1)],
                    Type::Record(vec![
                        (
                            Ident("data".to_string()),
                            Type::Ptr(Box::new(Type::Omit(1))),
                        ),
                        (Ident("length".to_string()), Type::I32),
                        (Ident("capacity".to_string()), Type::I32),
                    ]),
                ),
            ]
            .into_iter()
            .map(|(k, ps, v)| (Ident(k.to_string()), (ps, v)))
            .collect(),
            current_path: Path::empty(),
            imported: vec![],
            result_type: None,
            search_node: None,
            search_node_type: None,
            search_node_definition: None,
            completion: None,
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
                    self.globals.insert(
                        self.path_to(&func.name.data),
                        Source::transfer(func.to_type(), &func.name),
                    );
                }
                Decl::Let(ident, type_, _expr) => {
                    self.globals
                        .insert(self.path_to(&ident), Source::unknown(type_.clone()));
                }
                Decl::Type(ident, type_) => {
                    self.types.insert(ident.clone(), (vec![], type_.clone()));
                }
                Decl::Module(name, module) => {
                    let path = self.current_path.clone();
                    self.current_path.extend(name);
                    self.module_register_for_back_reference(module)?;
                    self.current_path = path;
                }
                Decl::Import(path) => {
                    self.imported.push(path.clone());
                }
            }
        }

        Ok(())
    }

    fn module_typecheck(&mut self, module: &mut Module) -> Result<()> {
        for decl in &mut module.0 {
            self.locals.clear();
            let result = self.decl(decl);
            if result.is_ok() || !self.search_node.is_some() {
                result?;
            }
        }

        Ok(())
    }

    fn decl(&mut self, decl: &mut Decl) -> Result<()> {
        match decl {
            Decl::Func(func) => {
                // For recursive functions
                self.globals.insert(
                    self.path_to(&func.name.data),
                    Source::transfer(func.to_type(), &func.name),
                );

                self.func(func)?;

                self.globals.insert(
                    self.path_to(&func.name.data),
                    Source::transfer(func.to_type(), &func.name),
                );
            }
            Decl::Let(ident, type_, expr) => {
                let mut result = self.expr(expr)?;
                self.unify(type_, &mut result).context(ErrorInSource {
                    path: Some(self.current_path.clone()),
                    start: expr.start.unwrap_or(0),
                    end: expr.end.unwrap_or(0),
                })?;

                self.globals
                    .insert(self.path_to(&ident), Source::unknown(type_.clone()));
            }
            Decl::Type(ident, type_) => {
                self.resolve_type(&type_)?;
                self.types.insert(ident.clone(), (vec![], type_.clone()));
            }
            Decl::Module(name, module) => {
                let module_path = self.current_path.clone();
                self.current_path.extend(name);
                self.module(module)?;
                self.current_path = module_path;
            }
            Decl::Import(_) => (),
        }

        Ok(())
    }

    fn resolve_type(&self, type_: &Type) -> Result<()> {
        match type_ {
            Type::Omit(_) => {}
            Type::Nil => {}
            Type::Bool => {}
            Type::I32 => {}
            Type::U32 => {}
            Type::I64 => {}
            Type::Byte => {}
            Type::Func(ts, ret) => {
                for t in ts {
                    self.resolve_type(t)?;
                }
                self.resolve_type(ret)?;
            }
            Type::VariadicFunc(ts, ret, var) => {
                for t in ts {
                    self.resolve_type(t)?;
                }
                self.resolve_type(ret)?;
                self.resolve_type(var)?;
            }
            Type::Record(rs) => {
                for (_, t) in rs {
                    self.resolve_type(t)?;
                }
            }
            Type::Ident(i) => {
                if !self.types.contains_key(&i) {
                    bail!("type `{}` not found", i.0);
                }
            }
            Type::Ptr(p) => {
                self.resolve_type(p)?;
            }
            Type::Array(t, _) => {
                self.resolve_type(t)?;
            }
            Type::Vec(v) => {
                self.resolve_type(v)?;
            }
            Type::Range(r) => {
                self.resolve_type(r)?;
            }
            Type::Optional(t) => {
                self.resolve_type(t)?;
            }
            Type::Map(k, v) => {
                self.resolve_type(k)?;
                self.resolve_type(v)?;
            }
            Type::Or(a, b) => {
                self.resolve_type(a)?;
                self.resolve_type(b)?;
            }
            Type::Any => {}
        }

        Ok(())
    }

    fn func(&mut self, func: &mut Func) -> Result<()> {
        let locals = self.locals.clone();

        for (name, type_) in &mut func.params {
            self.locals
                .insert(name.clone(), Source::unknown(type_.clone()));
        }
        if let Some((name, type_)) = &mut func.variadic {
            assert!(matches!(type_, Type::Vec(_)), "variadic must be vec");

            self.locals
                .insert(name.clone(), Source::unknown(type_.clone()));
        }

        self.result_type = Some(func.result.clone());
        self.block(&mut func.body)?;
        if !matches!(self.result_type, Some(Type::Nil)) && self.can_escape_block(&func.body) {
            return Err(
                anyhow!("function must return a value").context(ErrorInSource {
                    path: Some(self.current_path.clone()),
                    start: func.name.start.unwrap_or(0),
                    end: func.name.end.unwrap_or(0),
                }),
            );
        }

        self.locals = locals;

        if func.result.is_omit() {
            func.result = Type::Nil;
        }

        Ok(())
    }

    fn block(&mut self, statements: &mut Vec<Source<Statement>>) -> Result<()> {
        for statement in statements {
            self.statement(statement)?;
        }

        Ok(())
    }

    fn can_escape_block(&self, statements: &Vec<Source<Statement>>) -> bool {
        let mut can_escape = true;
        for statement in statements {
            match &statement.data {
                Statement::Return(_) => {
                    can_escape = false;
                }
                Statement::If(_, _, then_block, else_block) => {
                    if !self.can_escape_block(then_block) {
                        if let Some(else_block) = else_block {
                            if !self.can_escape_block(else_block) {
                                can_escape = false;
                            }
                        }
                    }
                }
                Statement::While(_, body) => {
                    can_escape = self.can_escape_block(body);
                }
                Statement::For(_, _, _, body) => {
                    can_escape = self.can_escape_block(body);
                }
                Statement::Let(_, _, _) => {}
                Statement::Expr(_, _) => {}
                Statement::Assign(_, _, _) => {}
                Statement::Continue => {}
                Statement::Break => {
                    return true;
                }
            }
        }

        can_escape
    }

    fn statement(&mut self, statement: &mut Source<Statement>) -> Result<()> {
        if let Some(cursor) = self.search_node.clone() {
            if self.is_search_finished(statement, &cursor) {
                return Ok(());
            }
        }

        match &mut statement.data {
            Statement::Let(pattern, type_, expr) => {
                let mut result = self.expr(expr)?;
                self.unify(type_, &mut result).context(ErrorInSource {
                    path: Some(self.current_path.clone()),
                    start: expr.start.unwrap_or(0),
                    end: expr.end.unwrap_or(0),
                })?;

                self.register_locals_with_pattern(&pattern, type_)?;
            }
            Statement::Return(expr) => {
                let type_ = self.expr(expr)?;

                let result_type = self.result_type.as_mut().unwrap();
                let result =
                    TypeChecker::unify_fn(result_type, &mut type_.clone()).context(ErrorInSource {
                        path: Some(self.current_path.clone()),
                        start: expr.start.unwrap_or(0),
                        end: expr.end.unwrap_or(0),
                    });
                if let Err(err) = result {
                    // return a : A or B --> return a or _
                    if let Type::Or(left, right) = self.result_type.as_mut().unwrap() {
                        if TypeChecker::unify_fn(left, &mut type_.clone()).is_ok() {
                            *expr = Source::transfer(
                                Expr::EnumOr(
                                    *left.clone(),
                                    *right.clone(),
                                    Some(Box::new(expr.clone())),
                                    None,
                                ),
                                expr,
                            );
                        } else {
                            return Err(err);
                        }
                    } else {
                        return Err(err);
                    }
                }
            }
            Statement::Expr(expr, type_) => {
                *type_ = self.expr(expr)?;

                if matches!(type_, Type::Or(_, _)) {
                    return Err(anyhow!("or type is not handled correctly").context(
                        ErrorInSource {
                            path: Some(self.current_path.clone()),
                            start: expr.start.unwrap_or(0),
                            end: expr.end.unwrap_or(0),
                        },
                    ));
                }
            }
            Statement::Assign(lhs, ast_rhs_type, rhs) => {
                let mut lhs_type = self.expr_left_value(lhs)?;
                let rhs_type = self.expr(rhs)?;
                let mut rhs_type_wrapped = Type::Ptr(Box::new(rhs_type.clone()));
                self.unify(&mut lhs_type, &mut rhs_type_wrapped)
                    .context(ErrorInSource {
                        path: Some(self.current_path.clone()),
                        start: lhs.start.unwrap_or(0),
                        end: lhs.end.unwrap_or(0),
                    })?;

                *ast_rhs_type = rhs_type;
            }
            Statement::If(cond, type_, then_block, else_block) => {
                let mut cond_type = self.expr(cond)?;
                self.unify(&mut cond_type, &mut Type::Bool)
                    .context(ErrorInSource {
                        path: Some(self.current_path.clone()),
                        start: cond.start.unwrap_or(0),
                        end: cond.end.unwrap_or(0),
                    })?;

                self.block(then_block).context("then block")?;

                if let Some(else_block) = else_block {
                    self.block(else_block).context("else block")?;
                }

                self.unify(type_, &mut Type::Nil)
                    .context("if::type_")
                    .context(ErrorInSource {
                        path: Some(self.current_path.clone()),
                        start: cond.start.unwrap_or(0),
                        end: cond.end.unwrap_or(0),
                    })?;
            }
            Statement::While(cond, block) => {
                let mut cond_type = self.expr(cond)?;
                self.unify(&mut cond_type, &mut Type::Bool)
                    .context(ErrorInSource {
                        path: Some(self.current_path.clone()),
                        start: cond.start.unwrap_or(0),
                        end: cond.end.unwrap_or(0),
                    })?;

                self.block(block)?;
            }
            Statement::For(mode, ident, range, body) => {
                let type_ = self.expr(range)?;
                if let Type::Range(range_type) = type_ {
                    *mode = Some(ForMode::Range);

                    self.locals
                        .insert(ident.clone(), Source::transfer(*range_type.clone(), range));

                    let locals = self.locals.clone();

                    self.block(body)?;

                    self.locals = locals;
                } else if let Type::Vec(vec_type) = type_ {
                    *mode = Some(ForMode::Vec(*vec_type.clone()));

                    self.locals
                        .insert(ident.clone(), Source::transfer(*vec_type.clone(), range));

                    let locals = self.locals.clone();

                    self.block(body)?;

                    self.locals = locals;
                } else {
                    return Err(
                        anyhow!("for range must be a range type").context(ErrorInSource {
                            path: Some(self.current_path.clone()),
                            start: range.start.unwrap_or(0),
                            end: range.end.unwrap_or(0),
                        }),
                    );
                }
            }
            Statement::Continue => (),
            Statement::Break => (),
        }

        Ok(())
    }

    fn register_locals_with_pattern(
        &mut self,
        pattern: &Source<Pattern>,
        type_: &Type,
    ) -> Result<()> {
        match &pattern.data {
            Pattern::Ident(ident) => {
                self.locals
                    .insert(ident.clone(), Source::transfer(type_.clone(), pattern));
                self.set_search_node_type(type_.clone(), pattern);
            }
            Pattern::Or(left, right) => match type_ {
                // let a or b = A or B
                // --> a: A?, b: B?
                Type::Or(left_type, right_type) => {
                    self.register_locals_with_pattern(
                        left,
                        &Type::Optional(Box::new(*left_type.clone())),
                    )?;
                    self.register_locals_with_pattern(
                        right,
                        &Type::Optional(Box::new(*right_type.clone())),
                    )?;
                }
                _ => bail!(
                    "Expected or type, got: {:?} in {:?} or {:?}",
                    type_,
                    left,
                    right
                ),
            },
            Pattern::Omit => todo!(),
        }

        Ok(())
    }

    fn resolve_path(&mut self, user_specified_path: &mut Path) -> Result<(Path, Source<Type>)> {
        let mut candidates = self.imported.clone();
        candidates.push(self.current_path.clone());

        for path_prefix in candidates {
            let mut path = path_prefix.clone();
            path.extend(user_specified_path);

            if let Ok(type_) = self.ident_path(&path) {
                return Ok((path, type_));
            }
        }

        bail!("Could not resolve path: {:?}", user_specified_path)
    }

    fn expr(&mut self, expr: &mut Source<Expr>) -> Result<Type> {
        match &mut expr.data {
            Expr::Lit(lit) => {
                let t = self.lit(lit)?;
                self.set_search_node_type(t.clone(), expr);

                Ok(t)
            }
            Expr::Ident {
                ident,
                resolved_path,
            } => {
                if let Ok(type_) = self.ident_local(ident) {
                    self.set_search_node_type(type_.data.clone(), expr);
                    self.set_search_node_definition(self.current_path.clone(), &type_, expr);
                    self.set_completion(type_.data.clone(), expr);

                    return Ok(type_.data);
                }

                let mut candidates = self.imported.clone();
                candidates.push(self.current_path.clone());

                for path_prefix in candidates {
                    let mut path = path_prefix.clone();
                    path.push(ident.clone());

                    if let Ok(type_) = self.ident_path(&path) {
                        *resolved_path = Some(path);

                        self.set_search_node_type(type_.data.clone(), expr);
                        self.set_search_node_definition(self.current_path.clone(), &type_, expr);
                        return Ok(type_.data);
                    }
                }

                let t = self.ident_global(ident).context(ErrorInSource {
                    path: Some(self.current_path.clone()),
                    start: expr.start.unwrap_or(0),
                    end: expr.end.unwrap_or(0),
                })?;

                self.set_search_node_type(t.data.clone(), expr);
                self.set_search_node_definition(self.current_path.clone(), &t, expr);
                Ok(t.data)
            }
            Expr::Path {
                path,
                resolved_path,
            } => {
                let (r, t) = self.resolve_path(&mut path.data).context(ErrorInSource {
                    path: Some(self.current_path.clone()),
                    start: path.start.unwrap_or(0),
                    end: path.end.unwrap_or(0),
                })?;
                *resolved_path = Some(r.clone());

                self.set_search_node_type(t.data.clone(), expr);
                self.set_search_node_definition(
                    Path::new(
                        // FIXME: need to strip current package path
                        r.0[0..2].to_vec(),
                    ),
                    &t,
                    expr,
                );
                Ok(t.data)
            }
            Expr::Call(caller, args, variadic_info, expansion) => {
                let caller_type = self.expr(caller)?;

                match caller_type {
                    Type::Func(mut arg_types, result_type) => {
                        if arg_types.len() != args.len() {
                            return Err(anyhow!(
                                "wrong number of arguments, expected {}, but found {}",
                                arg_types.len(),
                                args.len()
                            )
                            .context(ErrorInSource {
                                path: Some(self.current_path.clone()),
                                start: caller.start.unwrap_or(0),
                                end: caller.end.unwrap_or(0),
                            }));
                        }

                        for (index, arg) in args.into_iter().enumerate() {
                            let mut arg_type = self.expr(arg)?;
                            self.unify(&mut arg_types[index], &mut arg_type).context(
                                ErrorInSource {
                                    path: Some(self.current_path.clone()),
                                    start: arg.start.unwrap_or(0),
                                    end: arg.end.unwrap_or(0),
                                },
                            )?
                        }

                        if expansion.is_some() {
                            return Err(anyhow!("cannot expand a function call").context(
                                ErrorInSource {
                                    path: Some(self.current_path.clone()),
                                    start: caller.start.unwrap_or(0),
                                    end: caller.end.unwrap_or(0),
                                },
                            ));
                        }

                        Ok(result_type.as_ref().clone())
                    }
                    Type::VariadicFunc(mut arg_types, mut variadic, result_type) => {
                        if arg_types.len() > args.len() {
                            return Err(anyhow!(
                                "wrong number of arguments, expected at least {}, but found {}",
                                arg_types.len(),
                                args.len()
                            )
                            .context(ErrorInSource {
                                path: Some(self.current_path.clone()),
                                start: caller.start.unwrap_or(0),
                                end: caller.end.unwrap_or(0),
                            }));
                        }

                        let mut variadic_element = variadic.as_vec_type_element()?.clone();

                        for (index, arg) in args.into_iter().enumerate() {
                            let mut arg_type = self.expr(arg)?;
                            if index < arg_types.len() {
                                self.unify(&mut arg_types[index], &mut arg_type).context(
                                    ErrorInSource {
                                        path: Some(self.current_path.clone()),
                                        start: arg.start.unwrap_or(0),
                                        end: arg.end.unwrap_or(0),
                                    },
                                )?
                            } else {
                                self.unify(&mut variadic_element, &mut arg_type).context(
                                    ErrorInSource {
                                        path: Some(self.current_path.clone()),
                                        start: arg.start.unwrap_or(0),
                                        end: arg.end.unwrap_or(0),
                                    },
                                )?
                            }
                        }

                        if let Some(expansion) = expansion {
                            let mut expansion_type = self.expr(expansion)?;
                            self.unify(&mut variadic, &mut expansion_type).context(
                                ErrorInSource {
                                    path: Some(self.current_path.clone()),
                                    start: expansion.start.unwrap_or(0),
                                    end: expansion.end.unwrap_or(0),
                                },
                            )?
                        }

                        *variadic_info = Some(VariadicCall {
                            index: arg_types.len(),
                            element_type: variadic_element,
                        });

                        Ok(result_type.as_ref().clone())
                    }
                    _ => {
                        return Err(anyhow!("not a function").context(ErrorInSource {
                            path: Some(self.current_path.clone()),
                            start: caller.start.unwrap_or(0),
                            end: caller.end.unwrap_or(0),
                        }))
                    }
                }
            }
            Expr::BinOp(op, type_, arg1, arg2) => {
                use crate::ast::BinOp::*;

                let mut arg1_type = self.expr(arg1)?;
                let mut arg2_type = self.expr(arg2)?;
                self.unify(&mut arg1_type, &mut arg2_type)
                    .context(ErrorInSource {
                        path: Some(self.current_path.clone()),
                        start: arg1.start.unwrap_or(0),
                        end: arg1.end.unwrap_or(0),
                    })?;

                match op {
                    Add | Sub | Mul | Mod | Div => {
                        if !arg1_type.is_integer_type() {
                            bail!("Expected integer type, got {:?}", arg1_type)
                        }

                        *type_ = arg1_type.clone();

                        Ok(arg1_type)
                    }
                    And | Or => {
                        if !arg1_type.is_bool_type() {
                            bail!("Expected bool type, got {:?}", arg1_type)
                        }

                        *type_ = arg1_type.clone();

                        Ok(arg1_type)
                    }
                    Lt | Lte | Gt | Gte => {
                        if !arg1_type.is_integer_type() {
                            bail!("Expected integer type, got {:?}", arg1_type)
                        }

                        *type_ = arg1_type.clone();

                        Ok(Type::Bool)
                    }
                    Equal | NotEqual => {
                        *type_ = arg1_type.clone();

                        Ok(Type::Bool)
                    }
                }
            }
            Expr::Record(ident, record, expansion) => {
                let mut record_types = self
                    .resolve_record_type(Type::Ident(ident.data.clone()), vec![])
                    .context(ErrorInSource {
                        path: Some(self.current_path.clone()),
                        start: expr.start.unwrap_or(0),
                        end: expr.end.unwrap_or(0),
                    })?
                    .into_iter()
                    .collect::<HashMap<_, _>>();

                for (field, type_) in &mut record_types {
                    if let Some((_, expr)) = record.iter_mut().find(|(f, _)| f == field) {
                        let mut expr_type = self.expr(expr)?;
                        self.unify(type_, &mut expr_type).context(ErrorInSource {
                            path: Some(self.current_path.clone()),
                            start: expr.start.unwrap_or(0),
                            end: expr.end.unwrap_or(0),
                        })?;
                    } else {
                        if let Some(expansion) = expansion {
                            let mut expr_type = self.expr(expansion)?;
                            self.unify(type_, &mut expr_type).context(ErrorInSource {
                                path: Some(self.current_path.clone()),
                                start: expansion.start.unwrap_or(0),
                                end: expansion.end.unwrap_or(0),
                            })?;
                        } else {
                            return Err(anyhow!("missing field: {}", field.as_str()).context(
                                ErrorInSource {
                                    path: Some(self.current_path.clone()),
                                    start: expr.start.unwrap_or(0),
                                    end: expr.end.unwrap_or(0),
                                },
                            ));
                        }
                    }
                }

                for (field, expr) in record {
                    if !record_types.contains_key(&field) {
                        return Err(anyhow!("unknown field: {}", field.as_str()).context(
                            ErrorInSource {
                                path: Some(self.current_path.clone()),
                                start: expr.start.unwrap_or(0),
                                end: expr.end.unwrap_or(0),
                            },
                        ));
                    }
                }

                Ok(Type::Ident(ident.data.clone()))
            }
            Expr::AnonymousRecord(record, type_) => {
                let mut record_types = vec![];

                for (field, expr) in record {
                    let type_ = self.expr(expr)?;
                    record_types.push((field.clone(), type_));
                }

                *type_ = Type::Record(record_types);

                Ok(type_.clone())
            }
            Expr::Project(project_expr, type_, label_path) => {
                let mut expr_type = self.expr(project_expr)?;
                self.unify(type_, &mut expr_type).context(ErrorInSource {
                    path: Some(self.current_path.clone()),
                    start: project_expr.start.unwrap_or(0),
                    end: project_expr.end.unwrap_or(0),
                })?;

                let label = label_path.data.0.last().unwrap();

                // methods for builtin types
                if let Type::Ptr(p) = &mut expr_type {
                    match label.as_str() {
                        "at" => {
                            return Ok(Type::Func(vec![Type::I32], p.clone()));
                        }
                        "offset" => {
                            return Ok(Type::Func(vec![Type::I32], Box::new(Type::Ptr(p.clone()))));
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
                        "extend" => {
                            return Ok(Type::Func(
                                vec![Type::Vec(Box::new(*p.clone()))],
                                Box::new(Type::Nil),
                            ));
                        }
                        _ => (),
                    }
                }
                if let Type::Map(key, value) = &mut expr_type {
                    match label.as_str() {
                        "insert" => {
                            return Ok(Type::Func(
                                vec![key.as_ref().clone(), value.as_ref().clone()],
                                Box::new(Type::Nil),
                            ));
                        }
                        "at" => {
                            return Ok(Type::Func(vec![key.as_ref().clone()], value.clone()));
                        }
                        "has" => {
                            return Ok(Type::Func(
                                vec![key.as_ref().clone()],
                                Box::new(Type::Bool),
                            ));
                        }
                        _ => (),
                    }
                }

                // allow non-record type here
                // (some types only have methods, no fields)
                let fields = self
                    .resolve_record_type(expr_type.clone(), vec![])
                    .map(|v| v.into_iter().collect::<HashMap<_, _>>())
                    .unwrap_or(HashMap::new());

                if fields.contains_key(&label) {
                    // field access

                    let t = fields[label].clone();
                    self.set_search_node_type(t.clone(), expr);

                    Ok(t)
                } else {
                    // method access
                    let mut path = Path::new(vec![
                        expr_type.clone().to_ident().context(ErrorInSource {
                            path: Some(self.current_path.clone()),
                            start: project_expr.start.unwrap_or(0),
                            end: project_expr.end.unwrap_or(0),
                        })?,
                        label.clone(),
                    ]);
                    let (resolved_path, defined_type) = self.resolve_path(&mut path)?;
                    label_path.data = resolved_path.clone();
                    let type_ = defined_type.data.clone();

                    self.set_search_node_type(type_.clone(), label_path);
                    self.set_search_node_definition(
                        Path::new(
                            // FIXME: need to strip current package path
                            resolved_path.0[0..2].to_vec(),
                        ),
                        &defined_type,
                        label_path,
                    );

                    match type_ {
                        Type::Func(mut arg_types, result_type) => {
                            if arg_types.is_empty() {
                                return Err(anyhow!(
                                    "method {} has no arguments",
                                    label_path.data.as_str()
                                )
                                .context(ErrorInSource {
                                    path: Some(self.current_path.clone()),
                                    start: project_expr.start.unwrap_or(0),
                                    end: project_expr.end.unwrap_or(0),
                                }));
                            }
                            self.unify(&mut expr_type, &mut arg_types[0]).context(
                                ErrorInSource {
                                    path: Some(self.current_path.clone()),
                                    start: project_expr.start.unwrap_or(0),
                                    end: project_expr.end.unwrap_or(0),
                                },
                            )?;

                            Ok(Type::Func(arg_types[1..].to_vec(), result_type))
                        }
                        Type::VariadicFunc(mut arg_types, variadic, result_type) => {
                            if arg_types.is_empty() {
                                return Err(anyhow!(
                                    "method {} has no arguments",
                                    label_path.data.as_str()
                                )
                                .context(ErrorInSource {
                                    path: Some(self.current_path.clone()),
                                    start: project_expr.start.unwrap_or(0),
                                    end: project_expr.end.unwrap_or(0),
                                }));
                            }
                            self.unify(&mut expr_type, &mut arg_types[0]).context(
                                ErrorInSource {
                                    path: Some(self.current_path.clone()),
                                    start: project_expr.start.unwrap_or(0),
                                    end: project_expr.end.unwrap_or(0),
                                },
                            )?;

                            Ok(Type::VariadicFunc(arg_types, variadic, result_type))
                        }
                        _ => {
                            return Err(anyhow!(
                                "method {} is not a function",
                                label_path.data.as_str()
                            )
                            .context(ErrorInSource {
                                path: Some(self.current_path.clone()),
                                start: project_expr.start.unwrap_or(0),
                                end: project_expr.end.unwrap_or(0),
                            }));
                        }
                    }
                }
            }
            Expr::Make(type_, args) => {
                if matches!(type_, Type::Ptr(_)) {
                    assert_eq!(args.len(), 1);

                    let mut type_ = self.expr(&mut args[0])?;
                    self.unify(&mut type_, &mut Type::I32)?;
                }
                if let Type::Vec(v) = type_ {
                    for arg in args {
                        let mut arg_t = self.expr(arg)?;
                        self.unify(&mut arg_t, v)?;
                    }
                }

                Ok(type_.clone())
            }
            Expr::Range(start, end) => {
                let mut start_type = self.expr(start)?;
                let mut end_type = self.expr(end)?;

                self.unify(&mut start_type, &mut end_type)
                    .context(ErrorInSource {
                        path: Some(self.current_path.clone()),
                        start: start.start.unwrap_or(0),
                        end: start.end.unwrap_or(0),
                    })?;

                Ok(Type::Range(Box::new(start_type)))
            }
            Expr::As(expr, source, target) => {
                let source_type = self.expr(expr)?;
                *source = source_type.clone();

                Ok(target.clone())
            }
            Expr::SizeOf(_) => Ok(Type::I32),
            Expr::Self_ => {
                // FIXME: This is not working for caller expression

                if self.current_path.0.len() == 0 {
                    return Err(anyhow!("invalid self in empty module_path").context(
                        ErrorInSource {
                            path: Some(self.current_path.clone()),
                            start: expr.start.unwrap_or(0),
                            end: expr.end.unwrap_or(0),
                        },
                    ));
                }
                let ident = self.current_path.0.last().unwrap();

                // FIXME: adhoc type convertion
                Ok(match ident.as_str() {
                    "i32" => Type::I32,
                    _ => Type::Ident(ident.clone()),
                })
            }
            Expr::Wrap(type_, expr) => {
                let expr_type = self.expr(expr)?;
                *type_ = expr_type.clone();

                Ok(Type::Optional(Box::new(expr_type)))
            }
            Expr::Unwrap(type_, mode, expr) => {
                let mut expr_type = self.expr(expr)?;

                if matches!(expr_type, Type::Optional(_)) {
                    *mode = Some(UnwrapMode::Optional);

                    let mut wrapped_type = Type::Optional(Box::new(type_.clone()));
                    self.unify(&mut wrapped_type, &mut expr_type)
                        .context(ErrorInSource {
                            path: Some(self.current_path.clone()),
                            start: expr.start.unwrap_or(0),
                            end: expr.end.unwrap_or(0),
                        })?;

                    let wrapperd_type_element = *wrapped_type.to_optional()?.clone();
                    *type_ = wrapperd_type_element.clone();

                    Ok(wrapperd_type_element)
                } else if matches!(expr_type, Type::Or(_, _)) {
                    *mode = Some(UnwrapMode::Or);

                    let mut wrapped_type =
                        Type::Or(Box::new(type_.clone()), Box::new(Type::Omit(0)));
                    self.unify(&mut wrapped_type, &mut expr_type)
                        .context(ErrorInSource {
                            path: Some(self.current_path.clone()),
                            start: expr.start.unwrap_or(0),
                            end: expr.end.unwrap_or(0),
                        })?;

                    let (wrapperd_type_element, _) = wrapped_type.as_or_type()?;
                    *type_ = wrapperd_type_element.clone();

                    Ok(wrapperd_type_element.clone())
                } else {
                    Err(anyhow!("unwrap type mismatch").context(ErrorInSource {
                        path: Some(self.current_path.clone()),
                        start: expr.start.unwrap_or(0),
                        end: expr.end.unwrap_or(0),
                    }))
                }
            }
            Expr::Omit(_) => todo!(),
            Expr::EnumOr(lhs_type, rhs_type, lhs, rhs) => {
                if let Some(lhs) = lhs {
                    *lhs_type = self.expr(lhs)?;
                } else {
                    *lhs_type = Type::Any;
                };
                if let Some(rhs) = rhs {
                    *rhs_type = self.expr(rhs)?;
                } else {
                    *rhs_type = Type::Any;
                };

                Ok(Type::Or(
                    Box::new(lhs_type.clone()),
                    Box::new(rhs_type.clone()),
                ))
            }
            Expr::Try(expr) => {
                let expr_type = self.expr(expr)?;

                match expr_type {
                    Type::Or(lhs, rhs) => {
                        self.unify(
                            &mut self.result_type.clone().unwrap(),
                            &mut Type::Or(Box::new(Type::Omit(0)), rhs),
                        )
                        .context(ErrorInSource {
                            path: Some(self.current_path.clone()),
                            start: expr.start.unwrap_or(0),
                            end: expr.end.unwrap_or(0),
                        })?;

                        Ok(*lhs)
                    }
                    t => {
                        return Err(
                            anyhow!("try type mismatch, {:?}", t).context(ErrorInSource {
                                path: Some(self.current_path.clone()),
                                start: expr.start.unwrap_or(0),
                                end: expr.end.unwrap_or(0),
                            }),
                        )
                    }
                }
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
            Lit::U32(_) => Ok(Type::U32),
            Lit::I64(_) => Ok(Type::I64),
            Lit::String(_) => Ok(Type::Ident(Ident("string".to_string()))),
        }
    }

    fn ident_local(&mut self, ident: &mut Ident) -> Result<Source<Type>> {
        match self.locals.get(ident) {
            Some(type_) => Ok(type_.clone()),
            None => bail!("Ident Not Found: {}", ident.as_str()),
        }
    }

    fn ident_global(&mut self, ident: &mut Ident) -> Result<Source<Type>> {
        match self.globals.get(&Path::ident(ident.clone())) {
            Some(type_) => Ok(type_.clone()),
            None => bail!("Ident Not Found: {}", ident.as_str()),
        }
    }

    fn ident_path(&mut self, path: &Path) -> Result<Source<Type>> {
        match self.globals.get(path) {
            Some(type_) => Ok(type_.clone()),
            None => bail!("Path Not Found: {}", path.as_str()),
        }
    }

    fn unify(&mut self, type1: &mut Type, type2: &mut Type) -> Result<()> {
        let cs = Constrains::unify(type1, type2)?;
        cs.apply(type1);
        cs.apply(type2);

        Ok(())
    }

    fn unify_fn(type1: &mut Type, type2: &mut Type) -> Result<()> {
        let cs = Constrains::unify(type1, type2)?;
        cs.apply(type1);
        cs.apply(type2);

        Ok(())
    }

    fn resolve_record_type(&mut self, type_: Type, args: Vec<Type>) -> Result<Vec<(Ident, Type)>> {
        match type_ {
            Type::Ident(ident) => {
                let (params, mut type_) = self
                    .types
                    .get(&ident)
                    .ok_or(anyhow!("unknown type: {}", ident.as_str()))?
                    .clone();

                let mut apps = vec![];
                for (param, arg) in params.into_iter().zip(args.into_iter()) {
                    let Type::Omit(p) = param else {
                        bail!("expected omit type, but found {}", param.to_string());
                    };

                    apps.push((p, arg));
                }
                let cs = Constrains::new_from_hashmap(apps.into_iter().collect::<HashMap<_, _>>());
                cs.apply(&mut type_);

                let fields = type_.clone().to_record()?;

                Ok(fields)
            }
            Type::Record(fields) => Ok(fields),
            Type::Array(_, _) => {
                self.resolve_record_type(Type::Ident(Ident("array".to_string())), vec![])
            }
            Type::Vec(t) => {
                self.resolve_record_type(Type::Ident(Ident("vec".to_string())), vec![*t.clone()])
            }
            _ => bail!("expected record type, but found {}", type_.to_string()),
        }
    }

    fn path_to(&self, ident: &Ident) -> Path {
        let mut path = self.current_path.clone();
        path.push(ident.clone());

        path
    }

    // For LSP
    pub fn find_at_cursor(
        &mut self,
        module: &mut Module,
        path: Path,
        cursor: usize,
    ) -> Result<Option<Type>> {
        self.search_node = Some((path, cursor));

        let _ = self.module(module);

        Ok(self.search_node_type.clone())
    }

    fn set_search_node_type<T>(&mut self, type_: Type, source: &Source<T>) {
        if let Some((path, cursor)) = &self.search_node {
            if is_prefix_vec(&path.0, &self.current_path.0) {
                if let None = self.search_node_type {
                    if source.start.unwrap_or(0) <= *cursor && *cursor <= source.end.unwrap_or(0) {
                        self.search_node_type = Some(type_);
                    }
                }
            }
        }
    }

    fn is_search_finished<T>(&mut self, expr: &mut Source<T>, node: &(Path, usize)) -> bool {
        if self.search_node_type.is_some() {
            return true;
        }
        if let Some(start) = expr.start {
            if start > node.1 {
                return true;
            }
        }

        false
    }

    pub fn find_definition(
        &mut self,
        module: &mut Module,
        path: Path,
        cursor: usize,
    ) -> Result<Option<(Path, usize, usize)>> {
        self.search_node = Some((path, cursor));

        let _ = self.module(module);

        Ok(self.search_node_definition.clone())
    }

    fn set_search_node_definition<S, T>(
        &mut self,
        path: Path,
        def: &Source<S>,
        source: &Source<T>,
    ) {
        if let Some((search_path, cursor)) = &self.search_node {
            if &self.current_path == search_path {
                if source.start.unwrap_or(0) <= *cursor && *cursor <= source.end.unwrap_or(0) {
                    if let None = self.search_node_definition {
                        match (def.start, def.end) {
                            (Some(start), Some(end)) => {
                                self.search_node_definition = Some((path, start, end));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    pub fn completion(
        &mut self,
        module: &mut Module,
        path: Path,
        cursor: usize,
        dot: bool,
    ) -> Result<Option<Vec<(String, String, String)>>> {
        self.search_node = Some((path, cursor));

        let _ = self.module(module);

        if !dot {
            self.completion = Some(
                self.globals
                    .clone()
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            "function".to_string(),
                            k.remove_prefix(&Path::new(vec![
                                Ident("quartz".to_string()),
                                Ident("std".to_string()),
                            ]))
                            .as_str()
                            .to_string(),
                            v.data.to_string(),
                        )
                    })
                    .collect::<Vec<_>>(),
            );
        }

        Ok(self.completion.clone())
    }

    fn set_completion<T>(&mut self, type_: Type, source: &Source<T>) {
        if let Some((path, cursor)) = self.search_node.clone() {
            if is_prefix_vec(&path.0, &self.current_path.0) {
                // UGLY HACK: For dot completion, some nodes are skipped. So we need to search nodes for a bit wider range.
                if source.start.unwrap_or(0) <= cursor && cursor <= source.end.unwrap_or(0) + 5 {
                    let mut completion = vec![];

                    // field completion
                    if let Ok(rs) = self.resolve_record_type(type_.clone(), vec![]) {
                        for (field, type_) in rs {
                            completion.push(("field".to_string(), field.0, type_.to_string()));
                        }
                    }

                    // method completion
                    if let Ok(ident) = type_.to_ident() {
                        let search_path = Path::ident(ident.clone());

                        for mut import_path in self.imported.clone() {
                            import_path.extend(&search_path);
                            for (k, v) in &self.globals {
                                if k.starts_with(&import_path) {
                                    let label = k.remove_prefix(&import_path);

                                    completion.push((
                                        "function".to_string(),
                                        label.0[0].clone().0,
                                        v.data.to_string(),
                                    ));
                                }
                            }
                        }
                    }

                    self.completion = Some(completion);
                }
            }
        }
    }
}

fn is_prefix_vec<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.len() <= b.len() && a.iter().zip(b).all(|(x, y)| x == y)
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
            (Type::Optional(type1), Type::Optional(type2)) => {
                Constrains::unify(type1.as_mut(), type2.as_mut())
            }
            (Type::Bool, Type::Ident(ident)) if ident.as_str() == "bool" => Ok(Constrains::empty()),
            (Type::Ident(ident), Type::Bool) if ident.as_str() == "bool" => Ok(Constrains::empty()),
            (Type::I64, Type::Ident(ident)) if ident.as_str() == "i64" => Ok(Constrains::empty()),
            (Type::Ident(ident), Type::I64) if ident.as_str() == "i64" => Ok(Constrains::empty()),
            (Type::Or(type1, type2), Type::Or(type3, type4)) => {
                let mut constrains = Constrains::empty();
                constrains.merge(&Constrains::unify(type1.as_mut(), type3.as_mut())?);
                constrains.merge(&Constrains::unify(type2.as_mut(), type4.as_mut())?);

                Ok(constrains)
            }
            (_, Type::Any) => Ok(Constrains::empty()),
            (Type::Any, _) => Ok(Constrains::empty()),
            (type1, type2) => {
                bail!(
                    "type mismatch, expected {}, but found {}",
                    type1.to_string(),
                    type2.to_string()
                );
            }
        }
    }

    pub fn new_from_hashmap(constrains: HashMap<usize, Type>) -> Constrains {
        Constrains { constrains }
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
            Type::VariadicFunc(args, variadic, result) => {
                for arg in args {
                    self.apply(arg);
                }
                self.apply(variadic);
                self.apply(result.as_mut());
            }
            Type::Nil => {}
            Type::Bool => {}
            Type::I32 => {}
            Type::U32 => {}
            Type::I64 => {}
            Type::Byte => {}
            Type::Record(r) => {
                for (_, type_) in r {
                    self.apply(type_);
                }
            }
            Type::Ident(_) => {}
            Type::Ptr(p) => {
                self.apply(p);
            }
            Type::Array(t, _) => {
                self.apply(t);
            }
            Type::Vec(v) => {
                self.apply(v);
            }
            Type::Range(r) => {
                self.apply(r);
            }
            Type::Optional(t) => {
                self.apply(t);
            }
            Type::Map(k, v) => {
                self.apply(k);
                self.apply(v);
            }
            Type::Or(t1, t2) => {
                self.apply(t1);
                self.apply(t2);
            }
            Type::Any => {}
        }
    }
}

#[test]
fn test_unify() {
    let cases = vec![
        (
            Type::Optional(Box::new(Type::I32)),
            Type::Optional(Box::new(Type::Omit(0))),
            vec![(0, Type::I32)],
        ),
        (
            Type::Optional(Box::new(Type::Ident(Ident("foo".to_string())))),
            Type::Optional(Box::new(Type::Omit(0))),
            vec![(0, Type::Ident(Ident("foo".to_string())))],
        ),
    ];

    for (mut type1, mut type2, result) in cases {
        let cs = Constrains::unify(&mut type1, &mut type2).unwrap();

        assert_eq!(
            cs,
            Constrains::new_from_hashmap(result.into_iter().collect())
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        compiler::Compiler,
        typecheck::TypeChecker,
        util::{ident::Ident, path::Path},
    };

    #[test]
    fn test_typechecker_fail() {
        let cases = vec![
            r#"
struct Foo {
    a: not_defined,
}

fun main() {}
"#,
            r#"
fun main(): i32 {
}
"#,
            r#"
struct Foo {
    a: i32,
}

fun main() {
    let f = Foo { a: "hello" };
}
"#,
            r#"
fun main(): string? {
    return "foo";
}
"#,
            r#"
fun main(): i32? {
    return 0;
}
"#,
            r#"
fun f(): nil or error {
    return nil;
}

fun main() {
    f();
}
"#,
        ];
        for input in cases {
            let mut compiler = Compiler::new();
            let mut parsed = compiler
                .parse("", Path::ident(Ident("main".to_string())), input, true)
                .unwrap();

            let mut typechecker = TypeChecker::new();
            let result = typechecker.run(&mut parsed);
            assert!(result.is_err());
        }
    }
}
