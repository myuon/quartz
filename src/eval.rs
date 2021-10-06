use std::collections::HashMap;

use anyhow::Result;

use crate::ast::{DataValue, Declaration, Expr, Module, Statement};

pub struct Evaluator {
    variables: HashMap<String, DataValue>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            variables: HashMap::new(),
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
            Statement::Let(_, x, e) => {
                let value = self.eval_expr(e)?;
                self.variables.insert(x.to_string(), value);
            }
            Statement::Expr(e) => {
                self.eval_expr(e)?;
            }
            Statement::Return(e) => {
                return Ok(self.eval_expr(e)?);
            }
            Statement::ReturnIf(_, _) => todo!(),
            Statement::If(_, _, _) => todo!(),
            Statement::Continue => todo!(),
        }

        Ok(DataValue::Nil)
    }

    pub fn eval_expr(&mut self, expr: Expr) -> Result<DataValue> {
        match expr {
            Expr::Var(v) => self.load(&v),
            Expr::Lit(lit) => Ok(lit.into_datatype()),
            Expr::Fun(_, _, _) => todo!(),
            Expr::Call(f, args) => {
                let (eargs, statements) = self.load(&f)?.as_closure()?;
                let args = args
                    .into_iter()
                    .map(|arg| self.eval_expr(arg))
                    .collect::<Result<Vec<_>>>()?;

                let variables_snapshot = self.variables.clone();

                self.variables.extend(
                    eargs
                        .into_iter()
                        .zip(args)
                        .map(|(name, value)| (name.clone(), value)),
                );

                let mut result = DataValue::Nil;
                for stmt in statements {
                    result = self.eval_statement(stmt)?;
                }

                self.variables = variables_snapshot;

                Ok(result)
            }
            Expr::Ref(_) => todo!(),
            Expr::Deref(_) => todo!(),
            Expr::Loop(_) => todo!(),
        }
    }

    pub fn eval_decl(&mut self, decl: Declaration) -> Result<DataValue> {
        match decl {
            Declaration::Function(func) => {
                self.variables
                    .insert(func.name.clone(), DataValue::Closure(func.args, func.body));
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
                r#"
                    fn main() {
                        return 10;
                    }
                "#,
                DataValue::Int(10),
            ),
            (
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
