use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::ast::{Expr, Literal, Module, Statement, Type};

pub struct TypeChecker {
    variables: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            variables: HashMap::new(),
        }
    }

    fn unify(&mut self, t1: &Type, t2: &Type) -> Result<()> {
        match (t1, t2) {
            (Type::Infer, t) => Ok(()),
            (t, Type::Infer) => Ok(()),
            (Type::Any, _) => Ok(()),
            (_, Type::Any) => Ok(()),
            (Type::Unit, Type::Unit) => Ok(()),
            (Type::Int, Type::Int) => Ok(()),
            (Type::Bool, Type::Bool) => Ok(()),
            (t1, t2) => bail!("Type error, want {:?} but found {:?}", t1, t2),
        }
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
                Literal::Nil => Ok(Type::Ref(Box::new(Type::Any))),
                Literal::Bool(_) => Ok(Type::Bool),
                Literal::Int(_) => Ok(Type::Int),
                Literal::String(_) => Ok(Type::String),
            },
            Expr::Fun(_, args, body) => {
                let variables = self.variables.clone();
                let mut arg_types = vec![];
                for arg in args {
                    arg_types.push(Type::Infer);
                    self.variables.insert(arg.clone(), Type::Any);
                }

                let ret_type = self.statements(body)?;
                self.variables = variables;

                Ok(Type::Fn(arg_types, Box::new(ret_type)))
            }
            Expr::Call(f, args) => {
                let fn_type = self.load(f)?;
                let (arg_types, ret_type) = fn_type.as_fn_type().ok_or(anyhow::anyhow!(
                    "Expected function type but found: {:?}",
                    fn_type
                ))?;

                for (e, t) in args.iter_mut().zip(arg_types) {
                    let e_type = self.expr(e)?;
                    self.unify(&e_type, t)?;
                }

                Ok(ret_type.as_ref().clone())
            }
            Expr::Ref(e) => {
                let t = self.expr(e.as_mut())?;

                Ok(Type::Ref(Box::new(t)))
            }
            Expr::Deref(e) => {
                let t = self.expr(e.as_mut())?;

                let t_inner = t
                    .as_ref_type()
                    .ok_or(anyhow::anyhow!("Expected ref type but found: {:?}", t))?;

                Ok(t_inner.as_ref().clone())
            }
            Expr::Loop(body) => {
                self.statements(body)?;

                Ok(Type::Unit)
            }
        }
    }

    pub fn statements(&mut self, statements: &mut Vec<Statement>) -> Result<Type> {
        let mut ret_type = Type::Unit;

        for statement in statements {
            match statement {
                Statement::Let(_, x, body) => {
                    let body_type = self.expr(body)?;
                    self.variables.insert(x.clone(), body_type);
                }
                Statement::Expr(e) => {
                    self.expr(e)?;
                }
                Statement::Return(t) => {
                    ret_type = self.expr(t)?;
                }
                Statement::ReturnIf(_, _) => todo!(),
                Statement::If(cond, then_statements, else_statements) => {
                    let cond_type = self.expr(cond.as_mut())?;
                    self.unify(&cond_type, &Type::Bool)?;

                    let then_type = self.statements(then_statements)?;
                    let else_type = self.statements(else_statements)?;
                    self.unify(&then_type, &else_type)?;

                    ret_type = then_type;
                }
                Statement::Continue => {}
            }
        }

        Ok(ret_type)
    }

    pub fn module(&mut self, module: &mut Module) -> Result<Type> {
        self.statements(&mut module.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::run_parser;

    use super::*;

    #[test]
    fn test_typecheck() {
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
                // function type
                r#"
                    let f = fn (a, b, c) {
                        return c;
                    };
                    f(1, nil, "foo");
                "#,
                vec![(
                    "f",
                    Type::Fn(
                        vec![Type::Int, Type::Ref(Box::new(Type::Any)), Type::String],
                        Box::new(Type::String),
                    ),
                )],
                Type::String,
            ),
        ];

        for c in cases {
            let mut module = run_parser(c.0).unwrap();
            let mut typechecker = TypeChecker::new();
            let result = typechecker.module(&mut module).unwrap();

            assert_eq!(
                typechecker.variables,
                c.1.into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect::<HashMap<_, _>>()
            );
            assert_eq!(result, c.2, "{}", c.0);
        }
    }
}
