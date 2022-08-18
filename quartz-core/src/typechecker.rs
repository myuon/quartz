use std::collections::{HashMap, HashSet};

use anyhow::{bail, Context, Result};
use pretty_assertions::assert_eq;

use crate::{
    ast::{
        CallMode, Declaration, Expr, Literal, Module, OptionalMode, Source, Statement, Structs,
        Type,
    },
    compiler::specify_source_in_input,
};

#[derive(Debug)]
struct Constraints(Vec<(usize, Type)>);

impl Constraints {
    fn new() -> Constraints {
        Constraints(vec![])
    }

    fn insert(&mut self, index: usize, typ: Type) {
        self.0.push((index, typ));
    }

    fn singleton(index: usize, typ: Type) -> Constraints {
        let mut c = Constraints::new();
        c.insert(index, typ);

        c
    }

    fn unify(t1: &Type, t2: &Type) -> Result<Constraints> {
        match (t1, t2) {
            (t1, t2) if t1 == t2 => Ok(Constraints::new()),
            (Type::Infer(u), t) => Ok(Constraints::singleton(*u, t.clone())),
            (t, Type::Infer(u)) => Ok(Constraints::singleton(*u, t.clone())),
            (Type::Any, _) => Ok(Constraints::new()),
            (_, Type::Any) => Ok(Constraints::new()),
            (Type::Ref(s), Type::Ref(t)) => Ok(Constraints::unify(&s, &t)?),
            (Type::Fn(args1, ret1), Type::Fn(args2, ret2)) => {
                if args1.len() != args2.len() {
                    bail!("Type error: want {:?} but found {:?}", args1, args2);
                }

                let mut result = Constraints::new();
                for (arg1, arg2) in args1.iter().zip(args2) {
                    let cs = Constraints::unify(arg1, arg2)?;
                    result.merge(&cs)?;
                }

                let cs = Constraints::unify(ret1, ret2)?;
                result.merge(&cs)?;

                Ok(result)
            }
            (Type::Array(t1), Type::Array(t2)) => {
                let cs = Constraints::unify(t1, t2)?;
                Ok(cs)
            }
            // string == array[byte]
            (Type::Struct(s), Type::Array(t)) if s == "string" && t.as_ref() == &Type::Byte => {
                Ok(Constraints::new())
            }
            (Type::Array(t), Type::Struct(s)) if s == "string" && t.as_ref() == &Type::Byte => {
                Ok(Constraints::new())
            }
            // nil in byte
            (Type::Nil, Type::Byte) => Ok(Constraints::new()),
            // nil in ref type
            (Type::Nil, Type::Ref(_)) => Ok(Constraints::new()),
            // nil in optional type
            (Type::Nil, Type::Optional(_)) => Ok(Constraints::new()),
            (t1, t2) => bail!("Type error, want {:?} but found {:?}", t1, t2),
        }
    }

    fn find(&self, index: usize) -> Option<Type> {
        self.0
            .iter()
            .find(|(i, _)| *i == index)
            .map(|(_, t)| t.clone())
    }

    fn merge(&mut self, c2: &Constraints) -> Result<()> {
        for (k, v) in c2.0.iter() {
            if v.has_infer(*k) {
                bail!("Cyclic type dependency detected");
            }

            self.subst(*k, v);
            self.insert(*k, v.clone());
        }

        Ok(())
    }

    fn subst(&mut self, index: usize, typ: &Type) {
        for i in 0..self.0.len() {
            let (_, v) = self.0.get_mut(i).unwrap();

            v.subst(index, typ);
        }
    }

    fn apply(&self, typ: &mut Type) {
        match typ {
            Type::Infer(u) => {
                if let Some(t) = self.find(*u) {
                    *typ = t;
                }
            }
            Type::Any => {}
            Type::Nil => {}
            Type::Bool => {}
            Type::Int => {}
            Type::Fn(args, ret) => {
                for arg in args {
                    self.apply(arg);
                }
                self.apply(ret);
            }
            Type::Method(self_, args, ret) => {
                self.apply(self_);
                for arg in args {
                    self.apply(arg);
                }
                self.apply(ret);
            }
            Type::Struct(_) => {}
            Type::Ref(r) => {
                self.apply(r);
            }
            Type::Byte => {}
            Type::Array(arr) => {
                self.apply(arr);
            }
            Type::SizedArray(arr, _) => {
                self.apply(arr);
            }
            Type::Optional(t) => {
                self.apply(t);
            }
            Type::Self_ => {}
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeChecker<'s> {
    infer_count: usize,
    inferred: HashMap<usize, Type>,
    pub variables: HashMap<String, Type>,
    pub structs: Structs,
    pub function_types: HashMap<String, (Vec<Type>, Type)>,
    pub method_types: HashMap<(String, String), (Vec<Type>, Type)>,
    pub source_code: &'s str,
    call_graph: HashMap<String, HashMap<String, ()>>,
    struct_graph: HashMap<String, HashMap<String, ()>>,
    current_function: Option<String>,
    entrypoint: String,
    self_object: Option<Box<Source<Expr>>>,
}

impl<'s> TypeChecker<'s> {
    pub fn new(
        variables: HashMap<String, Type>,
        structs: Structs,
        source_code: &'s str,
    ) -> TypeChecker {
        TypeChecker {
            infer_count: 1,
            inferred: HashMap::new(),
            variables,
            structs,
            function_types: HashMap::new(),
            method_types: HashMap::new(),
            source_code,
            call_graph: HashMap::new(),
            struct_graph: HashMap::new(),
            current_function: None,
            entrypoint: "main".to_string(),
            self_object: None,
        }
    }

    pub fn set_entrypoint(&mut self, entrypoint: String) {
        self.entrypoint = entrypoint;
    }

    pub fn unify(&mut self, expected: &Type, actual: &mut Type) -> Result<()> {
        let cs = Constraints::unify(expected, actual)?;
        cs.apply(actual);
        self.inferred.extend(cs.0);

        Ok(())
    }

    fn error_context(
        &self,
        start: Option<usize>,
        end: Option<usize>,
        unknown_context: &str,
    ) -> String {
        if self.source_code.is_empty() {
            return unknown_context.to_string();
        }

        match (start, end) {
            (Some(start), Some(end)) => specify_source_in_input(self.source_code, start, end),
            _ => unknown_context.to_string(),
        }
    }

    fn next_infer(&mut self) -> Type {
        let t = Type::Infer(self.infer_count);
        self.infer_count += 1;

        t
    }

    fn normalize_type(&mut self, typ: &mut Type) {
        if let Type::Infer(0) = typ {
            *typ = self.next_infer();
        }
    }

    fn load(&mut self, v: &Vec<String>, typ: &mut Type) -> Result<()> {
        assert!(v.len() <= 2);
        if v.len() == 1 {
            let v = &v[0];

            if self.function_types.contains_key(v) {
                self.call_graph
                    .entry(self.current_function.clone().unwrap())
                    .or_insert(HashMap::new())
                    .insert(v.clone(), ());

                let f = self.function_types[v].clone();

                self.unify(&Type::Fn(f.0, Box::new(f.1)), typ)?;
            } else {
                let t = self
                    .variables
                    .get(v)
                    .ok_or(anyhow::anyhow!("Variable {} not found", v))?
                    .clone();

                self.unify(&t, typ)?;
            }
        } else {
            let (args, ret) = self
                .method_types
                .get(&(v[0].clone(), v[1].clone()))
                .ok_or(anyhow::anyhow!("Method {}::{} not found", v[0], v[1]))?;
            self.call_graph
                .entry(self.current_function.clone().unwrap())
                .or_insert(HashMap::new())
                .insert(format!("{}::{}", v[0], v[1]), ());

            self.unify(
                &Type::Method(
                    Box::new(Type::Struct(v[0].clone())),
                    args.clone(),
                    Box::new(ret.clone()),
                ),
                typ,
            )?;
        };

        Ok(())
    }

    fn transform(
        &self,
        expr: &mut Source<Expr>,
        current_type: &mut Type,
        expected_type: &Type,
    ) -> Result<()> {
        if let Type::Ref(current_type) = current_type {
            if let Type::Ref(expected_type) = expected_type {
                return self.transform(expr, current_type, expected_type);
            }
        }

        // return immediately if the types are already equal
        if Constraints::unify(current_type, expected_type).is_ok() {
            return Ok(());
        }

        // reference
        if !current_type.is_ref() && expected_type.is_ref() {
            *expr = Source::unknown(Expr::Address(Box::new(expr.clone()), current_type.clone()));
            *current_type = Type::Ref(Box::new(current_type.clone()));
        }
        // dereference
        if current_type.is_ref() && !expected_type.is_ref() {
            *expr = Source::unknown(Expr::Deref(Box::new(expr.clone()), expected_type.clone()));
            *current_type = current_type.clone().as_ref_type().unwrap().as_ref().clone();
        }
        // optional
        if !current_type.is_optional() && expected_type.is_optional() {
            if current_type.is_nil() {
                *expr = Source::unknown(Expr::Optional(
                    OptionalMode::Nil,
                    expected_type.clone(),
                    Box::new(expr.clone()),
                ));
                *current_type = expected_type.clone();
            } else {
                *expr = Source::unknown(Expr::Optional(
                    OptionalMode::Some,
                    expected_type.clone(),
                    Box::new(expr.clone()),
                ));
                *current_type = Type::Optional(Box::new(current_type.clone()));
            }
        }

        Constraints::unify(current_type, expected_type).context(self.error_context(
            expr.start,
            expr.end,
            "transform",
        ))?;

        Ok(())
    }

    fn reduce_to_callable(&self, expr: &mut Source<Expr>, typ: &mut Type) -> Result<()> {
        match typ {
            Type::Ref(t) => {
                *expr = Source::unknown(Expr::Deref(Box::new((*expr).clone()), t.as_ref().clone()));
                self.reduce_to_callable(expr, t)?;

                *typ = t.as_ref().clone();
            }
            Type::Method(_, _, _) | Type::Fn(_, _) | Type::Array(_) | Type::SizedArray(_, _) => {}
            Type::Struct(s) if s == "string" => {}
            t => bail!("Cannot call non-function type {:?}", t),
        };

        Ok(())
    }

    pub fn expr(&mut self, expr: &mut Source<Expr>, typ: &mut Type) -> Result<()> {
        match &mut expr.data {
            Expr::Var(v) => {
                self.load(v, typ)
                    .context(self.error_context(expr.start, expr.end, "var"))?;
            }
            Expr::Method(subj, v) => {
                self.load(&vec![subj.method_selector_name()?, v.clone()], typ)
                    .context(self.error_context(expr.start, expr.end, "var"))?;
            }
            Expr::Lit(lit, lit_typ) => {
                let t = match lit {
                    Literal::Bool(_) => Type::Bool,
                    Literal::Int(_) => Type::Int,
                    Literal::String(_) => Type::Struct("string".to_string()),
                    Literal::Nil => Type::Nil,
                    Literal::Array(arr, t) => {
                        for e in arr {
                            self.expr(e, t)?;
                        }

                        Type::Array(Box::new(t.clone()))
                    }
                };

                self.unify(&t, typ)
                    .context(self.error_context(expr.start, expr.end, "literal"))?;
                *lit_typ = typ.clone();
            }
            Expr::Call(mode, f, args) => {
                let mut fn_type = self.next_infer();
                self.expr(f, &mut fn_type)?;
                self.reduce_to_callable(f, &mut fn_type)?;

                if let Some((t, _)) = fn_type.as_sized_array() {
                    // array indexing
                    *mode = CallMode::SizedArray;

                    assert_eq!(args.len(), 1);
                    self.expr(&mut args[0], &mut Type::Int)?;
                    self.unify(t, typ).context(self.error_context(
                        expr.start,
                        expr.end,
                        "array indexing",
                    ))?;
                } else if let Some(t) = fn_type.as_array() {
                    // array indexing
                    *mode = CallMode::Array;

                    assert_eq!(args.len(), 1);
                    self.expr(&mut args[0], &mut Type::Int)?;
                    self.unify(t, typ).context(self.error_context(
                        expr.start,
                        expr.end,
                        "array indexing",
                    ))?;
                } else if let Some("string") = fn_type.as_struct_type().map(|s| s.as_str()) {
                    // string indexing
                    *mode = CallMode::Array;

                    self.expr(&mut args[0], &mut Type::Int)?;
                    self.unify(&Type::Byte, typ).context(self.error_context(
                        expr.start,
                        expr.end,
                        "string indexing",
                    ))?;
                } else {
                    // restore self_object here
                    if let Some(obj) = self.self_object.take() {
                        args.insert(0, obj.as_ref().clone());
                    }

                    let (arg_types, ret_type) = fn_type.as_fn_type().ok_or(anyhow::anyhow!(
                        "Cannot call non-function type {:?}",
                        fn_type
                    ))?;
                    let arg_types = arg_types.clone();

                    let actual_arg_len = args.len();
                    let expected_arg_len = if fn_type.is_method_type() {
                        // FIXME: -1
                        arg_types.len()
                    } else {
                        arg_types.len()
                    };
                    if expected_arg_len != actual_arg_len {
                        anyhow::bail!(
                            "Expected {} arguments but given {} for {:?}, {} (args: {:?}): {:?})",
                            expected_arg_len,
                            actual_arg_len,
                            f,
                            self.error_context(f.start, f.end, "no source"),
                            args,
                            fn_type,
                        );
                    }

                    for i in 0..actual_arg_len {
                        let mut current = self.next_infer();
                        self.expr_coerce(&mut args[i], &mut current, &arg_types[i])
                            .context(format!("{}th argument", i))?;
                    }

                    self.unify(&ret_type, typ)
                        .context(self.error_context(expr.start, expr.end, "call"))?;
                }
            }
            Expr::Struct(s, fields) => {
                assert_eq!(
                    self.structs.0.contains_key(s),
                    true,
                    "{}",
                    self.error_context(expr.start, expr.end, "")
                );

                let defined = self.structs.0[s]
                    .clone()
                    .into_iter()
                    .collect::<HashMap<String, Type>>();
                assert_eq!(defined.len(), fields.len());

                let first_fields = fields[0].clone().1;
                for (label, expr, typ) in fields {
                    self.expr_coerce(expr, typ, &defined[label])
                        .context(self.error_context(expr.start, expr.end, "struct field"))?;
                }

                self.struct_graph
                    .entry(self.current_function.clone().unwrap())
                    .or_insert(HashMap::new())
                    .insert(s.clone(), ());

                self.unify(&Type::Struct(s.clone()), typ)
                    .context(self.error_context(first_fields.start, first_fields.end, "struct"))?;
            }
            Expr::Project(is_method, proj_typ, proj, field) => {
                self.expr(proj, proj_typ)?;
                let name = proj_typ.method_selector_name().context(self.error_context(
                    proj.start,
                    proj.end,
                    &format!("[proj] {:?}", proj),
                ))?;

                if let Some((arg_types, return_type)) = self
                    .method_types
                    .get(&(name.clone(), field.clone()))
                    .cloned()
                {
                    // FIXME: if pointer, something could be go wrong

                    *is_method = true;

                    self.call_graph
                        .entry(self.current_function.clone().unwrap())
                        .or_insert(HashMap::new())
                        .insert(
                            // FIXME: use name_path for Func
                            format!("{}::{}", name, field),
                            (),
                        );

                    // DESUGAR: x.f(m) => X::f(x, m)
                    // x will be stored in self_object
                    // x is passed by ref
                    if !arg_types.is_empty() {
                        let mut self_object = proj.clone();
                        let mut current_type = proj_typ.clone();

                        self.transform(&mut self_object, &mut current_type, &arg_types[0])?;
                        self.self_object = Some(self_object);
                    }
                    let method_type = Type::Method(
                        Box::new(Type::Struct(name.clone())),
                        arg_types.clone(),
                        Box::new(return_type.clone()),
                    );
                    *expr = Source::unknown(Expr::Var(vec![name.clone(), field.clone()]));

                    self.unify(&method_type, typ)
                        .context(format!("[project] {:?}", expr))?;
                } else {
                    let field_type = self
                        .structs
                        .get_projection_type(&name, &field)
                        .context(self.error_context(proj.start, proj.end, "projection"))?;

                    *is_method = false;

                    self.unify(&field_type, typ).context(self.error_context(
                        proj.start,
                        proj.end,
                        "projection",
                    ))?;
                }
            }
            Expr::Ref(e, t) => {
                self.expr(e, t)?;
                self.unify(&Type::Ref(Box::new(t.clone())), typ)
                    .context(self.error_context(e.start, e.end, "ref"))?;
            }
            Expr::Deref(d, t) => {
                self.expr(d, t)?;
                self.unify(
                    t.as_ref_type()
                        .ok_or(anyhow::anyhow!("Cannot deref non-reference type {:?}", t))?,
                    typ,
                )
                .context(self.error_context(d.start, d.end, "deref"))?;
            }
            Expr::As(e, current_type, t) => {
                self.expr_coerce(e, current_type, t)?;
                self.unify(t, typ)
                    .context(self.error_context(e.start, e.end, "as"))?;
            }
            Expr::Address(e, t) => {
                self.expr(e, t)?;
                self.unify(&Type::Ref(Box::new(t.clone())), typ)
                    .context(self.error_context(e.start, e.end, "address"))?;
            }
            Expr::Make(t, args) => match t {
                Type::SizedArray(arr, _) => {
                    assert_eq!(args.len(), 1);
                    self.expr(&mut args[0], arr)?;
                    self.unify(t, typ)
                        .context(self.error_context(expr.start, expr.end, "make"))?;
                }
                Type::Array(arr) => {
                    if args.len() == 2 {
                        self.expr(&mut args[0], &mut Type::Int)?;
                        self.expr(&mut args[1], arr)?;
                        self.unify(t, typ)
                            .context(self.error_context(expr.start, expr.end, "make"))?;
                    } else {
                        bail!(
                            "Expected 2 arguments but given {:?}, {}",
                            args,
                            self.error_context(args[0].start, args[0].end, "make:array")
                        );
                    }
                }
                _ => unreachable!("new {:?} {:?}", t, args),
            },
            Expr::Optional(_, _, _) => {
                todo!()
            }
            Expr::Unwrap(expr, t) => {
                self.expr(expr, t)?;
                self.unify(t.unwrap_type()?, typ)
                    .context(self.error_context(expr.start, expr.end, "unwrap"))?;
            }
        };

        Ok(())
    }

    fn expr_coerce(
        &mut self,
        expr: &mut Source<Expr>,
        current_type: &mut Type,
        expected_typ: &Type,
    ) -> Result<()> {
        self.expr(expr, current_type)?;
        self.transform(expr, current_type, expected_typ)?;

        Ok(())
    }

    pub fn statement(
        &mut self,
        statement: &mut Source<Statement>,
        return_type: &mut Type,
    ) -> Result<()> {
        match &mut statement.data {
            Statement::Let(x, body, t) => {
                self.expr(body, t)?;
                self.variables.insert(x.clone(), t.clone());
            }
            Statement::Expr(e, t) => {
                self.expr(e, t)
                    .context(self.error_context(e.start, e.end, "expression"))?;
            }
            Statement::Return(e, t) => {
                self.expr(e, t).context(self.error_context(
                    statement.start,
                    statement.end,
                    "return",
                ))?;
                self.unify(t, return_type).context(self.error_context(
                    statement.start,
                    statement.end,
                    "return",
                ))?;
            }
            Statement::If(cond, then_statements, else_statements) => {
                let mut cond_typ = self.next_infer();
                self.expr_coerce(cond.as_mut(), &mut cond_typ, &Type::Bool)?;
                self.statements(then_statements, return_type)?;
                self.statements(else_statements, return_type)?;
            }
            Statement::Continue => {}
            Statement::Assignment(t, lhs, rhs) => {
                self.expr(lhs, t)?;

                let mut current = self.next_infer();
                self.expr_coerce(rhs, &mut current, &t)?;
            }
            Statement::While(cond, body) => {
                self.expr(cond, &mut Type::Bool)?;
                self.statements(body, return_type)?;
            }
        };

        Ok(())
    }

    pub fn statements(
        &mut self,
        statements: &mut Vec<Source<Statement>>,
        typ: &mut Type,
    ) -> Result<()> {
        // FIXME: support last expression as return type
        for i in 0..statements.len() {
            let statement = &mut statements[i];

            self.statement(statement, typ)?;
        }

        assert_eq!(
            self.self_object,
            None,
            "self_object {} in \n{}",
            self.error_context(
                self.self_object.clone().unwrap().start,
                self.self_object.clone().unwrap().end,
                ""
            ),
            self.error_context(statements[0].start, statements.last().unwrap().end, "")
        );

        Ok(())
    }

    pub fn function_statements(
        &mut self,
        statements: &mut Vec<Source<Statement>>,
        return_type: &mut Type,
    ) -> Result<()> {
        self.statements(statements, return_type)?;

        // Force nil type if the return_type is not inferred
        if let Type::Infer(_) = return_type {
            *return_type = Type::Nil;
        }

        Ok(())
    }

    pub fn declarations(&mut self, decls: &mut Vec<Declaration>) -> Result<()> {
        // preprocess: register all function types in this module
        for decl in decls.into_iter() {
            match decl {
                Declaration::Function(func) => {
                    let mut arg_types = vec![];
                    for arg in &func.args {
                        let t = if matches!(arg.1, Type::Infer(_)) {
                            self.next_infer()
                        } else {
                            arg.1.clone()
                        };

                        arg_types.push(t.clone());
                        self.variables.insert(arg.0.clone(), t);
                    }
                    self.normalize_type(&mut func.return_type);

                    self.function_types.insert(
                        func.name.data.clone(),
                        (arg_types.clone(), func.return_type.clone()),
                    );
                }
                Declaration::Method(typ, func) => {
                    let mut arg_types = vec![];
                    for arg in &mut func.args {
                        // NOTE: infer self type
                        if arg.1 == Type::Self_ {
                            arg.1 = Type::Ref(Box::new(typ.clone()));
                        }

                        let t = if matches!(arg.1, Type::Infer(_)) {
                            self.next_infer()
                        } else {
                            arg.1.clone()
                        };

                        arg_types.push(t.clone());
                        self.variables.insert(arg.0.clone(), t);
                    }
                    self.normalize_type(&mut func.return_type);

                    let key = (typ.method_selector_name()?, func.name.data.clone());
                    if self.method_types.contains_key(&key) {
                        bail!(
                            "Method {} already defined, {}",
                            func.name.data,
                            self.error_context(func.name.start, func.name.end, "function")
                        );
                    }
                    self.method_types
                        .insert(key, (arg_types.clone(), func.return_type.clone()));
                }
                _ => {}
            }
        }

        for decl in decls {
            self.current_function = decl.function_path();

            match decl {
                Declaration::Function(func) => {
                    let variables = self.variables.clone();
                    let mut arg_types = vec![];
                    for arg in &func.args {
                        let t = if matches!(arg.1, Type::Infer(_)) {
                            self.next_infer()
                        } else {
                            arg.1.clone()
                        };

                        arg_types.push(t.clone());
                        self.variables.insert(arg.0.clone(), t);
                    }

                    self.function_statements(&mut func.body, &mut func.return_type)?;
                    self.variables = variables;

                    self.function_types.get_mut(&func.name.data).unwrap().1 =
                        func.return_type.clone();
                }
                Declaration::Method(typ, func) => {
                    let variables = self.variables.clone();
                    let mut arg_types = vec![];
                    for arg in &func.args {
                        let t = if matches!(arg.1, Type::Infer(_)) {
                            self.next_infer()
                        } else {
                            arg.1.clone()
                        };

                        arg_types.push(t.clone());
                        self.variables.insert(arg.0.clone(), t);
                    }

                    if let Some(_) = typ.clone().as_sized_array() {
                        self.variables.insert("sized".to_string(), Type::Int);
                    }

                    self.function_statements(&mut func.body, &mut func.return_type)?;
                    self.variables = variables;

                    self.method_types
                        .get_mut(&(typ.method_selector_name()?, func.name.data.clone()))
                        .unwrap()
                        .1 = func.return_type.clone();
                }
                Declaration::Variable(x, e, typ) => {
                    self.expr(e, typ)?;
                    self.variables.insert(x.clone(), typ.clone());
                }
                Declaration::Struct(st) => {
                    assert!(!self.structs.0.contains_key(&st.name));
                    self.structs.0.insert(st.name.clone(), st.fields.clone());
                }
            }
        }

        Ok(())
    }

    pub fn module(&mut self, module: &mut Module) -> Result<()> {
        self.declarations(&mut module.0)?;

        // update dead_code fields for functions
        // calculate reachable functions from entrypoint
        let mut reachables = HashSet::new();
        let mut reachables_structs = HashSet::new();
        let mut stack = vec![self.entrypoint.as_str()];
        while let Some(func) = stack.pop() {
            if reachables.contains(&func) {
                continue;
            }

            reachables.insert(func);

            if let Some(h) = self.struct_graph.get(func) {
                reachables_structs.extend(h.keys());
            }

            if let Some(targets) = self.call_graph.get(func) {
                stack.extend(targets.keys().map(|f| f.as_str()));
            }
        }

        for decl in &mut module.0 {
            let function_path = decl.function_path();

            match decl {
                // TODO: support structs
                Declaration::Function(func) => {
                    if reachables.contains(function_path.unwrap().as_str()) {
                        continue;
                    }

                    func.dead_code = true;
                }
                Declaration::Method(_typ, func) => {
                    if reachables.contains(function_path.unwrap().as_str()) {
                        continue;
                    }

                    func.dead_code = true;
                }
                Declaration::Struct(s) => {
                    if reachables_structs.contains(&s.name) {
                        continue;
                    }

                    s.dead_code = true;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builtin::builtin,
        compiler::Compiler,
        parser::{run_parser, run_parser_statements},
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_typecheck_statements() {
        let cases = vec![(
            // primitive types
            r#"
                    let x = 5;
                    let y = "foo";
                    return y;
                "#,
            vec![("x", Type::Int), ("y", Type::Struct("string".to_string()))],
            Type::Struct("string".to_string()),
        )];

        for c in cases {
            let mut module = run_parser_statements(c.0).unwrap();
            let mut typechecker = TypeChecker::new(HashMap::new(), Structs(HashMap::new()), "");
            let mut result = Type::Infer(1);
            typechecker
                .statements(&mut module, &mut result)
                .expect(&format!("{}", c.0));

            assert_eq!(
                typechecker.variables,
                c.1.into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect::<HashMap<_, _>>()
            );
            assert_eq!(result, c.2, "{}", c.0);
        }
    }

    #[test]
    fn test_typecheck_fail() {
        let cases = vec![
            (
                r#"
                    func main() {
                        let x = 10;
                        x();
                    }
                "#,
                "Cannot call non-function type Int",
            ),
            (
                r#"
                    func f(a,b) {
                        return a;
                    }

                    func main() {
                        f(1);
                    }
                "#,
                "Expected 2 arguments but given 1",
            ),
        ];

        for c in cases {
            let mut module = run_parser(c.0).unwrap();
            let mut typechecker = TypeChecker::new(builtin(), Structs(HashMap::new()), "");
            let result = typechecker.module(&mut module);

            let err = result.unwrap_err();
            assert!(
                format!("{:?}", err).contains(c.1),
                "err: {:?}\n{}",
                err,
                c.0
            );
        }
    }

    #[test]
    fn test_typecheck() {
        let cases = vec![
            (
                // declare types for function arguments
                r#"
func f(a: int, b: string) {
    return b.len().eq(a);
}

func main() {
    return f(1,"hello");
}
                "#,
                vec![(
                    "f",
                    Type::Fn(
                        vec![Type::Int, Type::Struct("string".to_string())],
                        Box::new(Type::Bool),
                    ),
                )],
            ),
            (
                r#"
let a = 10;

func main() {
    a = 20;

    return a;
}
            "#,
                vec![],
            ),
            (
                r#"
func f() {
    g();
}

func g() {
    f();
}
            "#,
                vec![],
            ),
            (
                r#"
func f(n: int): int {
    return f(n - 1) + 1;
}
            "#,
                vec![],
            ),
            (
                r#"
func main(): int {
    let x = make[array[int,4]](0);
    x(0) = 1;
    x(1) = 2;
    x(2) = x(0) + x(1);

    return x(2);
}
            "#,
                vec![("main", Type::Fn(vec![], Box::new(Type::Int)))],
            ),
        ];

        for c in cases {
            let mut compiler = Compiler::new();
            let mut module = compiler
                .parse(
                    &(r#"
method string len(self): int {
    return 0;
}

method int eq(self, other: int): bool {
    return false;
}
"#
                    .to_string()
                        + c.0),
                )
                .unwrap();
            let mut checker = compiler.typecheck(&mut module).expect(&format!("{}", c.0));

            for (name, mut typ) in c.1 {
                checker.load(&vec![name.to_string()], &mut typ).unwrap();
            }
        }
    }
}
