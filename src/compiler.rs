use anyhow::Result;

use crate::{eval::Evaluator, parser::run_parser, vm::DataType};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn exec(&self, input: &str) -> Result<DataType> {
        let module = run_parser(input)?;

        let mut eval = Evaluator::new();
        eval.eval_module(module)
    }
}
