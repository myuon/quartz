use anyhow::Result;

use crate::{eval::Evaluator, parser::run_parser, stdlib::typecheck_with_stdlib, vm::DataType};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn exec(&self, input: &str) -> Result<DataType> {
        let mut module = run_parser(input)?;
        typecheck_with_stdlib(&mut module)?;

        let mut eval = Evaluator::new();
        eval.eval_module(module)
    }
}
