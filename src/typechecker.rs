use std::collections::HashMap;

use anyhow::{bail, Result};

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
            (Type::Any, _) => Ok(Constraints::new()),
            (_, Type::Any) => Ok(Constraints::new()),
            (Type::Infer(u), t) => Ok(Constraints::singleton(*u, t.clone())),
            (t, Type::Infer(u)) => Ok(Constraints::singleton(*u, t.clone())),
            (Type::Unit, Type::Unit) => Ok(Constraints::new()),
            (Type::Int, Type::Int) => Ok(Constraints::new()),
            (Type::Bool, Type::Bool) => Ok(Constraints::new()),
            (Type::String, Type::String) => Ok(Constraints::new()),
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
            (Type::Ref(t1), Type::Ref(t2)) => Constraints::unify(t1, t2),
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
            Type::Ref(typ_inner) => {
                self.apply(typ_inner);
            }
            Type::Fn(args, ret) => {
                for arg in args {
                    self.apply(arg);
                }
                self.apply(ret);
            }
        }
    }
}

pub struct TypeChecker {
    infer_count: usize,
    variables: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            infer_count: 0,
            variables: HashMap::new(),
        }
    }

    pub fn set_default_types(&mut self, m: HashMap<String, Type>) {
        self.variables = m;
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
        let t = self
            .variables
            .get(v)
            .ok_or(anyhow::anyhow!("Variable {} not found", v))?;

        Ok(t.clone())
    }

    pub fn expr(&mut self, expr: &mut Expr) -> Result<Type> {
        match expr {
            Expr::Var(v) => self.load(v),
            Expr::Lit(lit) => match lit {
                Literal::Nil => Ok(Type::Ref(Box::new(self.next_infer()))),
                Literal::Bool(_) => Ok(Type::Bool),
                Literal::Int(_) => Ok(Type::Int),
                Literal::String(_) => Ok(Type::String),
            },
            Expr::Fun(_, args, body) => {
                let variables = self.variables.clone();
                let mut arg_types = vec![];
                for arg in args {
                    let tvar = self.next_infer();

                    arg_types.push(tvar.clone());
                    self.variables.insert(arg.clone(), tvar);
                }

                let ret_type = self.statements(body)?;
                self.variables = variables;

                Ok(Type::Fn(arg_types, Box::new(ret_type)))
            }
            Expr::Call(f, args) => {
                let fn_type = self.load(f)?;
                if matches!(fn_type, Type::Any) {
                    return Ok(Type::Any);
                }

                fn_type.as_fn_type().ok_or(anyhow::anyhow!(
                    "Expected function type but found: {:?}",
                    fn_type
                ))?;

                let mut arg_types_inferred = vec![];
                for arg in args {
                    arg_types_inferred.push(self.expr(arg)?);
                }

                let mut ret_type_inferred = self.next_infer();

                let cs = Constraints::unify(
                    &fn_type,
                    &Type::Fn(arg_types_inferred, Box::new(ret_type_inferred.clone())),
                )?;

                self.apply_constraints(&cs);

                cs.apply(&mut ret_type_inferred);

                Ok(ret_type_inferred)
            }
            Expr::Ref(e) => {
                let t = self.expr(e.as_mut())?;

                Ok(Type::Ref(Box::new(t)))
            }
            Expr::Deref(e) => {
                let t = self.expr(e.as_mut())?;
                let mut t_inner = self.next_infer();

                let cs = Constraints::unify(&t, &Type::Ref(Box::new(t_inner.clone())))?;

                self.apply_constraints(&cs);

                cs.apply(&mut t_inner);

                Ok(t_inner)
            }
            Expr::Loop(body) => self.statements(body),
        }
    }

    pub fn statements(&mut self, statements: &mut Vec<Statement>) -> Result<Type> {
        let mut ret_type = Type::Any;

        for statement in statements {
            match statement {
                Statement::Let(_, x, body) => {
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
                Statement::ReturnIf(_, _) => todo!(),
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
            }
        }

        Ok(ret_type)
    }

    pub fn declarations(&mut self, decls: &mut Vec<Declaration>) -> Result<()> {
        for decl in decls {
            match decl {
                Declaration::Function(func) => {
                    let t = self.statements(&mut func.body)?;
                    self.variables
                        .insert(func.name.clone(), Type::Fn(vec![], Box::new(t)));
                }
            }
        }

        Ok(())
    }

    pub fn module(&mut self, module: &mut Module) -> Result<()> {
        self.declarations(&mut module.0)
    }
}

pub fn typecheck(module: &mut Module) -> Result<()> {
    let mut checker = TypeChecker::new();
    checker.module(module)?;

    Ok(())
}

pub fn typecheck_with(module: &mut Module, variables: HashMap<String, Type>) -> Result<()> {
    let mut checker = TypeChecker::new();
    checker.set_default_types(variables);
    checker.module(module)?;

    Ok(())
}

pub fn typecheck_statements(module: &mut Vec<Statement>) -> Result<()> {
    let mut checker = TypeChecker::new();
    checker.statements(module)?;

    Ok(())
}

pub fn typecheck_statements_with(
    module: &mut Vec<Statement>,
    variables: HashMap<String, Type>,
) -> Result<()> {
    let mut checker = TypeChecker::new();
    checker.set_default_types(variables);
    checker.statements(module)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::{run_parser, run_parser_statements};

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
                // unification for function type
                r#"
                    let f = fn (a, b, c) {
                        return c;
                    };
                    f(1, nil, "foo");
                "#,
                vec![(
                    "f",
                    Type::Fn(
                        vec![Type::Int, Type::Ref(Box::new(Type::Infer(3))), Type::String],
                        Box::new(Type::String),
                    ),
                )],
                Type::Unit,
            ),
            (
                r#"
                    return loop {
                        return 10;
                    };
                "#,
                vec![],
                Type::Int,
            ),
            (
                r#"
                    return loop {
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
            let mut typechecker = TypeChecker::new();
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
        let cases = vec![(
            r#"
            let x = 10;
            x();
        "#,
            "Expected function type but found",
        )];

        for c in cases {
            let mut module = run_parser_statements(c.0).unwrap();
            let mut typechecker = TypeChecker::new();
            let result = typechecker.statements(&mut module);

            let err = result.unwrap_err();
            assert!(err.to_string().contains(c.1), "err: {:?}\n{}", err, c.0);
        }
    }
}
