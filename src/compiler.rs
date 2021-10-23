use anyhow::Result;

use crate::{ast::DataValue, eval::Evaluator, parser::run_parser, stdlib::typecheck_with_stdlib};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn exec(&self, input: &str) -> Result<DataValue> {
        let mut module = run_parser(input)?;
        let checker = typecheck_with_stdlib(&mut module)?;

        let mut eval = Evaluator::new(checker.structs, checker.functions, checker.methods);
        eval.eval_module(module)
    }
}
