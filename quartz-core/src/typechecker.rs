use std::collections::HashMap;

use anyhow::{bail, Context, Result};

use crate::ast::{Declaration, Expr, Literal, Module, Statement, Type};

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
            Type::Unit => {}
            Type::Bool => {}
            Type::Int => {}
            Type::String => {}
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
        }
    }
}

pub struct TypeChecker {
    infer_count: usize,
    variables: HashMap<String, Type>,
    pub structs: HashMap<String, Vec<(String, Type)>>,
    pub functions: HashMap<
        String,
        (
            Vec<(String, Type)>, // argument types
            Box<Type>,           // return type
            Vec<Statement>,      // body
        ),
    >,
    pub methods: HashMap<
        (String, String), // receiver type, method name
        (
            String,              // receiver name
            Vec<(String, Type)>, // argument types
            Box<Type>,           // return type
            Vec<Statement>,      // body
        ),
    >,
}

impl TypeChecker {
    pub fn new(
        variables: HashMap<String, Type>,
        methods: HashMap<
            (String, String), // receiver type, method name
            (
                String,              // receiver name
                Vec<(String, Type)>, // argument types
                Box<Type>,           // return type
                Vec<Statement>,      // body
            ),
        >,
    ) -> TypeChecker {
        TypeChecker {
            infer_count: 0,
            variables,
            structs: HashMap::new(),
            functions: HashMap::new(),
            methods,
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

    fn load(&self, v: &String) -> Result<Type> {
        if self.functions.contains_key(v) {
            let f = self.functions[v].clone();
            Ok(Type::Fn(f.0.into_iter().map(|(_, v)| v).collect(), f.1))
        } else {
            let t = self
                .variables
                .get(v)
                .ok_or(anyhow::anyhow!("Variable {} not found", v))?;

            Ok(t.clone())
        }
    }

    pub fn expr(&mut self, expr: &mut Expr) -> Result<Type> {
        match expr {
            Expr::Var(v) => self.load(v),
            Expr::Lit(lit) => match lit {
                Literal::Bool(_) => Ok(Type::Bool),
                Literal::Int(_) => Ok(Type::Int),
                Literal::String(_) => Ok(Type::String),
                Literal::Nil => Ok(Type::Any),
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

                let mut ret_type_inferred = self.next_infer();

                let cs = Constraints::unify(
                    &fn_type,
                    &Type::Fn(
                        arg_types_inferred.clone(),
                        Box::new(ret_type_inferred.clone()),
                    ),
                )
                .context(format!(
                    "Unify {:?} & {:?} from {:?}",
                    fn_type,
                    Type::Fn(arg_types_inferred, Box::new(ret_type_inferred.clone())),
                    Expr::Call(f.clone(), args_cloned)
                ))?;

                self.apply_constraints(&cs);

                cs.apply(&mut ret_type_inferred);

                Ok(ret_type_inferred)
            }
            Expr::Struct(s, fields) => {
                assert!(self.structs.contains_key(s));

                let def = self.structs[s].clone();
                for ((k1, v1), (k2, v2)) in def.iter().zip(fields.into_iter()) {
                    assert_eq!(k1, k2);

                    let cs = Constraints::unify(&v1, &self.expr(v2)?)?;
                    self.apply_constraints(&cs);
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
                    Type::String => "string".to_string(),
                    Type::Any => "any".to_string(),
                    Type::Bool => "bool".to_string(),
                    _ => bail!("Cannot project of: {:?}", typ),
                };
                *name = typ_name.clone();

                if let Some(method) = self.methods.get(&(typ_name.clone(), field.clone())) {
                    // FIXME: if pointer, something could be go wrong

                    *is_method = true;

                    Ok(Type::Fn(
                        method.1.clone().into_iter().map(|(_, v)| v).collect(),
                        method.2.clone(),
                    ))
                } else {
                    let def = self
                        .structs
                        .get(&typ_name)
                        .ok_or_else(|| anyhow::anyhow!("{}.{} not found", typ_name, field))?;
                    let (_, field_type) = def
                        .iter()
                        .find(|(k, _)| k == field)
                        .ok_or(anyhow::anyhow!("Field {} not found", field))?;

                    *is_method = false;

                    Ok(if pointer {
                        Type::Ref(Box::new(field_type.clone()))
                    } else {
                        field_type.clone()
                    })
                }
            }
            Expr::Deref(d) => {
                let typ = self.expr(d)?;
                Ok(typ
                    .as_ref_type()
                    .ok_or(anyhow::anyhow!(
                        "Expected reference type but found: {:?}",
                        typ
                    ))?
                    .as_ref()
                    .clone())
            }
            Expr::Ref(e) => {
                let typ = self.expr(e)?;
                Ok(Type::Ref(Box::new(typ)))
            }
        }
    }

    pub fn statements(&mut self, statements: &mut Vec<Statement>) -> Result<Type> {
        let mut ret_type = Type::Any;

        for statement in statements {
            match statement {
                Statement::Let(x, body) => {
                    let body_type = self.expr(body)?;
                    self.variables.insert(x.clone(), body_type);
                    ret_type = Type::Unit;
                }
                Statement::Expr(e) => {
                    self.expr(e)?;
                    ret_type = Type::Unit;
                }
                Statement::Return(t) => {
                    ret_type = self.expr(t)?;
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

                    ret_type = Type::Unit;
                }
                Statement::Loop(body) => {
                    ret_type = self.statements(body)?;
                }
                Statement::While(cond, body) => {
                    let cond_type = self.expr(cond)?;
                    let cs = Constraints::unify(&cond_type, &Type::Bool)?;
                    self.apply_constraints(&cs);

                    let mut body_type = self.statements(body)?;
                    let cs = Constraints::unify(&body_type, &Type::Unit)?;
                    self.apply_constraints(&cs);

                    cs.apply(&mut body_type);

                    ret_type = body_type;
                }
            }
        }

        Ok(ret_type)
    }

    pub fn declarations(&mut self, decls: &mut Vec<Declaration>) -> Result<()> {
        for decl in decls {
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

                    let result_type = self.next_infer();

                    // 再帰関数の定義ができるように先にvariableに登録する
                    self.variables.insert(
                        func.name.clone(),
                        Type::Fn(arg_types.clone(), Box::new(result_type)),
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

                    let t = self.statements(&mut func.body)?;
                    self.variables = variables;

                    if let Some((name, typ, _pointer)) = func.method_of.clone() {
                        self.methods.insert(
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
                        self.functions.insert(
                            func.name.clone(),
                            (
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
                    }
                }
                Declaration::Variable(x, e) => {
                    let t = self.expr(e)?;
                    self.variables.insert(x.clone(), t);
                }
                Declaration::Struct(st) => {
                    assert!(!self.structs.contains_key(&st.name));
                    self.structs.insert(st.name.clone(), st.fields.clone());
                }
            }
        }

        Ok(())
    }

    pub fn module(&mut self, module: &mut Module) -> Result<()> {
        self.declarations(&mut module.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        compiler::Compiler,
        parser::{run_parser, run_parser_statements},
        stdlib::stdlib,
    };

    use super::*;

    #[test]
    fn test_typecheck_statements() {
        let cases = vec![
            (
                // primitive types
                r#"
                    let x = 5;
                    let y = "foo";
                    return y;
                "#,
                vec![("x", Type::Int), ("y", Type::String)],
                Type::String,
            ),
            (
                r#"
                    loop {
                        return 10;
                    };
                "#,
                vec![],
                Type::Int,
            ),
            (
                r#"
                    loop {
                        if true {
                            return 10;
                        };
                    };
                "#,
                vec![],
                Type::Int,
            ),
        ];

        for c in cases {
            let mut module = run_parser_statements(c.0).unwrap();
            let mut typechecker = TypeChecker::new(HashMap::new(), HashMap::new());
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
                    fn main() {
                        let x = 10;
                        x();
                    }
                "#,
                "Expected function type but found",
            ),
            (
                r#"
                    fn f(a,b) {
                        return a;
                    }

                    fn main() {
                        f(1);
                    }
                "#,
                "Expected 2 arguments but given 1",
            ),
        ];

        for c in cases {
            let mut module = run_parser(c.0).unwrap();
            let mut typechecker = TypeChecker::new(stdlib(), HashMap::new());
            let result = typechecker.module(&mut module);

            let err = result.unwrap_err();
            assert!(err.to_string().contains(c.1), "err: {:?}\n{}", err, c.0);
        }
    }

    #[test]
    fn test_typecheck() {
        let cases = vec![(
            // declare types for function arguments
            r#"
                    fn f(a: int, b: string) {
                        return b.len().eq(a);
                    }

                    fn main() {
                        return f(1,"hello");
                    }
                "#,
            vec![(
                "f",
                Type::Fn(vec![Type::Int, Type::String], Box::new(Type::Bool)),
            )],
        )];

        for c in cases {
            let compiler = Compiler::new();
            let mut module = compiler.parse(c.0).unwrap();
            let checker = compiler.typecheck(&mut module).expect(&format!("{}", c.0));

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