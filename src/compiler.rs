use anyhow::{Context, Result};

use crate::{
    ast::DataValue,
    eval::Evaluator,
    parser::run_parser,
    stdlib::{stdlib, stdlib_methods},
    typechecker::TypeChecker,
};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn exec(&self, input: &str) -> Result<DataValue> {
        let mut module = run_parser(input).context("Phase: parse")?;
        let mut checker = TypeChecker::new(stdlib(), stdlib_methods());
        checker.module(&mut module).context("Phase: typecheck")?;

        let mut eval = Evaluator::new(checker.structs, checker.functions, checker.methods);
        eval.eval_module(module).context("Phase: eval")
    }
}
