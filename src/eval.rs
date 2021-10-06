use std::collections::HashMap;

use anyhow::Result;

use crate::{
    ast::{Declaration, Expr, Module, Statement},
    vm::DataType,
};

struct Evaluator {
    variables: HashMap<String, DataType>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            variables: HashMap::new(),
        }
    }

    fn load(&self, name: &String) -> Result<DataType> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Variable {} not found", name))
    }

    pub fn eval_statement(&mut self, stmt: Statement) -> Result<DataType> {
        Ok(DataType::Nil)
    }

    pub fn eval_expr(&mut self, expr: Expr) -> Result<DataType> {
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

                let mut result = DataType::Nil;
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

    pub fn eval_decl(&mut self, decl: Declaration) -> Result<DataType> {
        match decl {
            Declaration::Function(func) => {
                self.variables.insert(
                    func.name.clone(),
                    DataType::Closure(0, func.args, func.body),
                );
            }
        }

        Ok(DataType::Nil)
    }

    pub fn eval_module(&mut self, m: Module) -> Result<DataType> {
        for decl in m.0 {
            self.eval_decl(decl)?;
        }

        self.eval_expr(Expr::Call(String::from("main"), vec![]))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        code_gen::{gen_code, gen_code_statements},
        parser::{run_parser, run_parser_statements},
        stdlib::{create_ffi_table, typecheck_statements_with_stdlib, typecheck_with_stdlib},
    };

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
                DataType::Int(10),
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
                DataType::Int(10),
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
