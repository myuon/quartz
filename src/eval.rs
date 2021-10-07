use std::collections::HashMap;

use anyhow::Result;

use crate::ast::{DataValue, Declaration, Expr, Module, Statement};

type NativeFunction = Box<dyn Fn(Vec<DataValue>) -> Result<DataValue>>;

fn new_native_functions() -> HashMap<String, NativeFunction> {
    let mut natives = HashMap::<String, NativeFunction>::new();
    natives.insert(
        "_add".to_string(),
        Box::new(|args| {
            Ok(DataValue::Int(
                args[0].clone().as_int()? + args[1].clone().as_int()?,
            ))
        }),
    );
    natives.insert(
        "_sub".to_string(),
        Box::new(|args| {
            Ok(DataValue::Int(
                args[0].clone().as_int()? - args[1].clone().as_int()?,
            ))
        }),
    );
    natives.insert(
        "_mult".to_string(),
        Box::new(|args| {
            Ok(DataValue::Int(
                args[0].clone().as_int()? * args[1].clone().as_int()?,
            ))
        }),
    );
    natives.insert(
        "_leq".to_string(),
        Box::new(|args| {
            Ok(DataValue::Bool(
                args[0].clone().as_int()? <= args[1].clone().as_int()?,
            ))
        }),
    );
    natives.insert(
        "_eq".to_string(),
        Box::new(|args| {
            Ok(DataValue::Bool(
                args[0].clone().as_int()? == args[1].clone().as_int()?,
            ))
        }),
    );

    natives
}

pub struct Evaluator {
    variables: HashMap<String, DataValue>,
    natives: HashMap<String, NativeFunction>,
    escape_return: Option<DataValue>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            variables: HashMap::new(),
            natives: new_native_functions(),
            escape_return: None,
        }
    }

    fn load(&self, name: &String) -> Result<DataValue> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Variable {} not found", name))
    }

    pub fn eval_statement(&mut self, stmt: Statement) -> Result<DataValue> {
        match stmt {
            Statement::Let(x, e) => {
                let value = self.eval_expr(e)?;
                self.variables.insert(x.to_string(), value);
            }
            Statement::Expr(e) => {
                self.eval_expr(e)?;
            }
            Statement::Return(e) => {
                self.escape_return = Some(self.eval_expr(e)?);
            }
            Statement::If(cond, body1, body2) => {
                let result = if self.eval_expr(cond.as_ref().clone())?.as_bool()? {
                    self.eval_statements(body1)?
                } else {
                    self.eval_statements(body2)?
                };

                return Ok(result);
            }
            Statement::Continue => todo!(),
            Statement::Assignment(x, e) => {
                let value = self.eval_expr(e)?;
                self.variables.insert(x.to_string(), value);
            }
        }

        Ok(DataValue::Nil)
    }

    pub fn eval_statements(&mut self, statements: Vec<Statement>) -> Result<DataValue> {
        let mut result = DataValue::Nil;
        for stmt in statements.clone() {
            result = self.eval_statement(stmt)?;
            if self.escape_return.is_some() {
                return Ok(DataValue::Nil);
            }
        }

        Ok(result)
    }

    pub fn eval_expr(&mut self, expr: Expr) -> Result<DataValue> {
        match expr {
            Expr::Var(v) => self.load(&v),
            Expr::Lit(lit) => Ok(lit.into_datatype()),
            Expr::Fun(_, _, _) => todo!(),
            Expr::Call(f, args) => {
                let args = args
                    .into_iter()
                    .map(|arg| self.eval_expr(arg))
                    .collect::<Result<Vec<_>>>()?;

                if let Some(func) = self.natives.get(&f) {
                    func(args)
                } else {
                    let (eargs, statements) = self.load(&f)?.as_closure()?;

                    let variables_snapshot = self.variables.clone();

                    self.variables.extend(
                        eargs
                            .into_iter()
                            .zip(args)
                            .map(|(name, value)| (name.clone(), value)),
                    );

                    let result = self.eval_statements(statements)?;
                    self.variables = variables_snapshot;

                    if let Some(ret) = self.escape_return.clone() {
                        assert_eq!(result, DataValue::Nil);
                        self.escape_return = None;

                        return Ok(ret);
                    }

                    Ok(result)
                }
            }
            Expr::Ref(_) => todo!(),
            Expr::Deref(_) => todo!(),
            Expr::Loop(body) => loop {
                self.eval_statements(body.clone())?;
                if self.escape_return.is_some() {
                    return Ok(DataValue::Nil);
                }
            },
        }
    }

    pub fn eval_decl(&mut self, decl: Declaration) -> Result<DataValue> {
        match decl {
            Declaration::Function(func) => {
                self.variables
                    .insert(func.name.clone(), DataValue::Closure(func.args, func.body));
            }
            Declaration::Variable(x, expr) => {
                let val = self.eval_expr(expr)?;
                self.variables.insert(x, val);
            }
        }

        Ok(DataValue::Nil)
    }

    pub fn eval_module(&mut self, m: Module) -> Result<DataValue> {
        for decl in m.0 {
            self.eval_decl(decl)?;
        }

        self.eval_expr(Expr::Call(String::from("main"), vec![]))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::run_parser, stdlib::typecheck_with_stdlib};

    use super::*;

    #[test]
    fn test_eval() -> Result<()> {
        let cases = vec![
            (
                // main
                r#"
                    fn main() {
                        return 10;
                    }
                "#,
                DataValue::Int(10),
            ),
            (
                // function call
                r#"
                    fn f() {
                        return 10;
                    }

                    fn main() {
                        return f();
                    }
                "#,
                DataValue::Int(10),
            ),
            (
                // add
                r#"
                    fn main() {
                        return _add(1, 2);
                    }
                "#,
                DataValue::Int(3),
            ),
            (
                // if
                r#"
                    fn check_bool(b) {
                        if b {
                            return 10;
                        } else {
                            return 20;
                        };
                    }

                    fn main() {
                        return check_bool(false);
                    }
                "#,
                DataValue::Int(20),
            ),
            (
                // recursion
                r#"
                    fn count_up(n) {
                        if _eq(n, 5) {
                            return true;
                        } else {
                            return count_up(_add(n, 1));
                        };
                    }

                    fn main() {
                        return count_up(1);
                    }
                "#,
                DataValue::Bool(true),
            ),
            (
                // factorial
                r#"
                    fn factorial(n) {
                        if _eq(n, 0) {
                            return 1;
                        } else {
                            return _mult(n, factorial(_sub(n, 1)));
                        };
                    }

                    fn main() {
                        return factorial(5);
                    }
                "#,
                DataValue::Int(120),
            ),
            (
                // global variables
                r#"
                    let x = 10;

                    fn main() {
                        return x;
                    }
                "#,
                DataValue::Int(10),
            ),
            (
                // reassignment
                r#"
                    fn f(b) {
                        let x = 0;

                        if b {
                            x = 10;
                        } else {
                        };

                        return x;
                    }

                    fn main() {
                        return f(true);
                    }
                "#,
                DataValue::Int(10),
            ),
            (
                // loop
                r#"
                    fn fib(n) {
                        let a = 1;
                        let b = 1;
                        let counter = 0;

                        loop {
                            if _eq(counter, n) {
                                return b;
                            };

                            let c = _add(a, b);
                            a = b;
                            b = c;

                            counter = _add(counter, 1);
                        };
                    }

                    fn main() {
                        return fib(10);
                    }
                "#,
                DataValue::Int(144),
            ),
        ];

        for (input, want) in cases {
            let mut m = run_parser(input)?;
            typecheck_with_stdlib(&mut m)?;

            let mut eval = Evaluator::new();
            assert_eq!(want, eval.eval_module(m)?, "{}", input);
        }

        Ok(())
    }
}
