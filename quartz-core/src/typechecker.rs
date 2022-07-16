use std::collections::{HashMap, HashSet};

use anyhow::{bail, Context, Result};
use pretty_assertions::assert_eq;

use crate::{
    ast::{Declaration, Expr, Literal, Methods, Module, Source, Statement, Structs, Type},
    compiler::specify_source_in_input,
};

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
            (Type::Any, _) => Ok(Constraints::new()),
            (_, Type::Any) => Ok(Constraints::new()),
            (Type::Infer(u), t) => Ok(Constraints::singleton(*u, t.clone())),
            (t, Type::Infer(u)) => Ok(Constraints::singleton(*u, t.clone())),
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
            (t1, t2) => bail!("Type error, want {:?} but found {:?}", t1, t2),
        }
    }

    fn coerce(t1: &Type, t2: &Type) -> Result<Constraints> {
        if let Ok(c) = Constraints::unify(t1, t2) {
            return Ok(c);
        }

        match (t1, t2) {
            (Type::Nil, Type::Optional(_)) => Ok(Constraints::new()),
            (t, Type::Optional(s)) => Constraints::unify(t, s),
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
            Type::Struct(_) => {}
            Type::Ref(r) => {
                self.apply(r);
            }
            Type::Byte => {}
            Type::Array(arr) => {
                self.apply(arr);
            }
            Type::Optional(t) => {
                self.apply(t);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeChecker<'s> {
    infer_count: usize,
    pub variables: HashMap<String, Type>,
    pub structs: Structs,
    pub function_types: HashMap<String, (Vec<Type>, Type)>,
    pub methods: Methods,
    pub source_code: &'s str,
    call_graph: HashMap<String, HashMap<String, ()>>,
    current_function: Option<String>,
    entrypoint: String,
}

impl<'s> TypeChecker<'s> {
    pub fn new(
        variables: HashMap<String, Type>,
        structs: Structs,
        methods: Methods,
        source_code: &'s str,
    ) -> TypeChecker {
        TypeChecker {
            infer_count: 1,
            variables,
            structs,
            function_types: HashMap::new(),
            methods,
            source_code,
            call_graph: HashMap::new(),
            current_function: None,
            entrypoint: "main".to_string(),
        }
    }

    pub fn set_entrypoint(&mut self, entrypoint: String) {
        self.entrypoint = entrypoint;
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

    fn apply_constraints(&mut self, constraints: &Constraints) {
        for key in self.variables.clone().keys() {
            self.variables.entry(key.clone()).and_modify(|typ| {
                constraints.apply(typ);
            });
        }
    }

    fn next_infer(&mut self) -> Type {
        let t = Type::Infer(self.infer_count);
        self.infer_count += 1;

        t
    }

    fn load(&mut self, v: &String) -> Result<Type> {
        if self.function_types.contains_key(v) {
            self.call_graph
                .entry(self.current_function.clone().unwrap())
                .or_insert(HashMap::new())
                .insert(v.clone(), ());

            let f = self.function_types[v].clone();
            Ok(Type::Fn(f.0, Box::new(f.1)))
        } else {
            let t = self
                .variables
                .get(v)
                .ok_or(anyhow::anyhow!("Variable {} not found", v))?;

            Ok(t.clone())
        }
    }

    pub fn expr(&mut self, expr: &mut Source<Expr>) -> Result<Type> {
        match &mut expr.data {
            Expr::Var(v, t) => {
                let vtype = self.load(v)?;
                *t = vtype.clone();
                Ok(vtype)
            }
            Expr::Lit(lit) => match lit {
                Literal::Bool(_) => Ok(Type::Bool),
                Literal::Int(_) => Ok(Type::Int),
                Literal::String(_) => Ok(Type::Struct("string".to_string())),
                Literal::Nil => Ok(Type::Nil),
                Literal::Array(arr, t) => {
                    for e in arr {
                        let etype = self.expr(e)?;
                        *t = etype.clone();
                    }

                    Ok(Type::Array(Box::new(t.clone())))
                }
            },
            Expr::Call(f, args) => {
                let fn_type = self.expr(f)?;
                if matches!(fn_type, Type::Any) {
                    return Ok(Type::Any);
                }

                fn_type.as_fn_type().ok_or(anyhow::anyhow!(
                    "Expected function type but found: {:?}",
                    fn_type
                ))?;

                let expected_arg_len = fn_type.as_fn_type().unwrap().0.len();
                let actual_arg_len = args.len();
                if expected_arg_len != actual_arg_len {
                    anyhow::bail!(
                        "Expected {} arguments but given {}",
                        expected_arg_len,
                        actual_arg_len
                    );
                }

                let mut arg_types_inferred = vec![];
                let args_cloned = args.clone();
                for arg in args {
                    arg_types_inferred.push(self.expr(arg)?);
                }

                let mut ret_type_inferred = fn_type.as_fn_type().unwrap().1.as_ref().clone();

                let cs = Constraints::unify(
                    &fn_type,
                    &Type::Fn(
                        arg_types_inferred.clone(),
                        Box::new(ret_type_inferred.clone()),
                    ),
                )
                .context(self.error_context(
                    f.start,
                    f.end,
                    &format!(
                        "Unify {:?} & {:?} from {:?}",
                        fn_type,
                        Type::Fn(arg_types_inferred, Box::new(ret_type_inferred.clone())),
                        Expr::Call(f.clone(), args_cloned)
                    ),
                ))?;

                self.apply_constraints(&cs);

                cs.apply(&mut ret_type_inferred);

                Ok(ret_type_inferred)
            }
            Expr::Struct(s, fields) => {
                assert_eq!(self.structs.0.contains_key(s), true);

                let def = self.structs.0[s].clone();
                for ((k1, v1), (k2, v2, t)) in def.iter().zip(fields.into_iter()) {
                    assert_eq!(k1, k2);

                    let mut result = self.expr(v2)?;
                    let cs = Constraints::coerce(&result, &v1)?;
                    self.apply_constraints(&cs);
                    cs.apply(&mut result);

                    *t = result;
                }

                Ok(Type::Struct(s.clone()))
            }
            Expr::Project(is_method, name, expr, field) => {
                let mut typ = self.expr(expr)?;

                let mut pointer = false;

                // FIXME: support nested refs
                if let Some(b) = typ.as_ref_type() {
                    typ = b.as_ref().clone();
                    pointer = true;
                }

                let typ_name = match typ.clone() {
                    Type::Struct(s) => s.clone(),
                    Type::Int => "int".to_string(),
                    Type::Any => "any".to_string(),
                    Type::Bool => "bool".to_string(),
                    _ => bail!("Cannot project of: {:?}", typ),
                };
                *name = typ_name.clone();

                if let Some(method) = self.methods.0.get(&(typ_name.clone(), field.clone())) {
                    // FIXME: if pointer, something could be go wrong

                    *is_method = true;

                    self.call_graph
                        .entry(self.current_function.clone().unwrap())
                        .or_insert(HashMap::new())
                        .insert(format!("{}::{}", name, field), ());

                    Ok(Type::Fn(
                        method.1.clone().into_iter().map(|(_, v)| v).collect(),
                        method.2.clone(),
                    ))
                } else {
                    let field_type = self
                        .structs
                        .get_projection_type(&typ_name, &field)
                        .context(self.error_context(expr.start, expr.end, "projection"))?;

                    *is_method = false;

                    Ok(if pointer {
                        Type::Ref(Box::new(field_type.clone()))
                    } else {
                        field_type.clone()
                    })
                }
            }
            Expr::Index(e, i) => {
                let mut r = self.next_infer();

                let typ = self.expr(e)?;
                let cs = Constraints::unify(&typ, &Type::Array(Box::new(r.clone())))?;
                self.apply_constraints(&cs);
                cs.apply(&mut r);

                let index_type = self.expr(i)?;
                let cs = Constraints::unify(&index_type, &Type::Int)?;
                self.apply_constraints(&cs);

                cs.apply(&mut r);

                Ok(r)
            }
        }
    }

    pub fn statements(&mut self, statements: &mut Vec<Source<Statement>>) -> Result<Type> {
        let mut ret_type = Type::Any;

        for i in 0..statements.len() {
            let statement = &mut statements[i];

            match &mut statement.data {
                Statement::Let(x, body, t) => {
                    let body_type = self.expr(body)?;
                    *t = body_type.clone();
                    self.variables.insert(x.clone(), body_type);
                    ret_type = Type::Nil;
                }
                Statement::Expr(e, t) => {
                    let etype =
                        self.expr(e)
                            .context(self.error_context(e.start, e.end, "expression"))?;
                    *t = etype;

                    ret_type = Type::Nil;
                }
                Statement::Return(e, t) => {
                    ret_type = self.expr(e).context(self.error_context(
                        statement.start,
                        statement.end,
                        "return",
                    ))?;
                    *t = ret_type.clone();
                }
                Statement::If(cond, then_statements, else_statements) => {
                    let cond_type = self.expr(cond.as_mut())?;
                    let cs = Constraints::unify(&cond_type, &Type::Bool)?;
                    self.apply_constraints(&cs);

                    let mut then_type = self.statements(then_statements)?;
                    let else_type = self.statements(else_statements)?;
                    let cs = Constraints::unify(&then_type, &else_type)?;
                    self.apply_constraints(&cs);

                    cs.apply(&mut then_type);

                    ret_type = then_type;
                }
                Statement::Continue => {}
                Statement::Assignment(lhs, rhs) => {
                    let typ = self.expr(lhs)?;
                    let cs = Constraints::unify(&typ, &self.expr(rhs)?)?;
                    self.apply_constraints(&cs);

                    ret_type = Type::Nil;
                }
                Statement::While(cond, body) => {
                    let cond_type = self.expr(cond)?;
                    let cs = Constraints::unify(&cond_type, &Type::Bool)?;
                    self.apply_constraints(&cs);

                    let mut body_type = self.statements(body)?;
                    let cs = Constraints::unify(&body_type, &Type::Nil)?;
                    self.apply_constraints(&cs);

                    cs.apply(&mut body_type);

                    ret_type = body_type;
                }
            }
        }

        Ok(ret_type)
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

                    if let Type::Infer(0) = func.return_type {
                        func.return_type = self.next_infer();
                    }

                    self.function_types.insert(
                        func.name.clone(),
                        (arg_types.clone(), func.return_type.clone()),
                    );

                    // メソッドのレシーバーも登録する
                    if let Some((name, typ, pointer)) = func.method_of.clone() {
                        self.variables.insert(
                            name.clone(),
                            if pointer {
                                Type::Ref(Box::new(Type::Struct(typ)))
                            } else {
                                Type::Struct(typ)
                            },
                        );
                    }
                }
                _ => {}
            }
        }

        for decl in decls {
            self.current_function = None;

            match decl {
                Declaration::Function(func) => {
                    self.current_function = Some(func.name_path());

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

                    if let Some((name, typ, pointer)) = func.method_of.clone() {
                        self.variables.insert(
                            name.clone(),
                            if pointer {
                                Type::Ref(Box::new(Type::Struct(typ)))
                            } else {
                                Type::Struct(typ)
                            },
                        );
                    }

                    let mut t = self.statements(&mut func.body)?;
                    let cs = Constraints::unify(&t, &func.return_type)?;
                    self.apply_constraints(&cs);
                    cs.apply(&mut t);
                    func.return_type = t.clone();

                    self.variables = variables;

                    // FIXME: move to preprocess phase
                    if let Some((name, typ, _pointer)) = func.method_of.clone() {
                        self.methods.0.insert(
                            (typ, func.name.clone()),
                            (
                                name,
                                func.args
                                    .iter()
                                    .map(|(name, _)| name)
                                    .cloned()
                                    .into_iter()
                                    .zip(arg_types.into_iter())
                                    .collect(),
                                Box::new(t.clone()),
                                func.body.clone(),
                            ),
                        );
                    } else {
                        self.function_types
                            .insert(func.name.clone(), (arg_types, t.clone()));
                    }
                }
                Declaration::Variable(x, e, typ) => {
                    let t = self.expr(e)?;
                    *typ = t.clone();
                    self.variables.insert(x.clone(), t);
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
        let mut stack = vec![self.entrypoint.as_str()];
        while let Some(func) = stack.pop() {
            if reachables.contains(&func) {
                continue;
            }

            reachables.insert(func);
            if let Some(targets) = self.call_graph.get(func) {
                stack.extend(targets.keys().map(|f| f.as_str()));
            }
        }

        for decl in &mut module.0 {
            match decl {
                // TODO: support structs
                Declaration::Function(func) => {
                    if reachables.contains(func.name_path().as_str()) {
                        continue;
                    }

                    func.dead_code = true;
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
            let mut typechecker = TypeChecker::new(
                HashMap::new(),
                Structs(HashMap::new()),
                Methods(HashMap::new()),
                "",
            );
            let result = typechecker
                .statements(&mut module)
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
                "Expected function type but found",
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
            let mut typechecker = TypeChecker::new(
                builtin(),
                Structs(HashMap::new()),
                Methods(HashMap::new()),
                "",
            );
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
func main(): byte {
    let x = _new(5);
    x[0] = _int_to_byte(1);
    x[1] = _int_to_byte(2);
    x[2] = _int_to_byte(_add(_byte_to_int(x[0]), _byte_to_int(x[1])));

    return x[2];
}
            "#,
                vec![("main", Type::Fn(vec![], Box::new(Type::Byte)))],
            ),
        ];

        for c in cases {
            let mut compiler = Compiler::new();
            compiler.typechecker.methods.0.insert(
                ("string".to_string(), "len".to_string()),
                ("len".to_string(), vec![], Box::new(Type::Int), vec![]),
            );
            compiler.typechecker.methods.0.insert(
                ("int".to_string(), "eq".to_string()),
                (
                    "x".to_string(),
                    vec![("y".to_string(), Type::Int)],
                    Box::new(Type::Bool),
                    vec![],
                ),
            );

            let mut module = compiler.parse(c.0).unwrap();
            let mut checker = compiler.typecheck(&mut module).expect(&format!("{}", c.0));

            for (name, typ) in c.1 {
                assert_eq!(
                    checker.load(&name.to_string()).unwrap(),
                    typ,
                    "{}\n{:?}",
                    c.0,
                    checker.variables
                );
            }
        }
    }
}
