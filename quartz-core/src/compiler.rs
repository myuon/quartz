use std::iter::repeat;

use anyhow::{Context, Error, Result};
use thiserror::Error as ThisError;

use crate::{
    ast::{DataValue, Module},
    eval::Evaluator,
    parser::run_parser,
    stdlib::{stdlib, stdlib_methods},
    typechecker::TypeChecker,
};

#[derive(ThisError, Debug)]
pub enum CompileError {
    #[error("parse error: {source:?}")]
    ParseError { source: Error, position: usize },
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn parse(&self, input: &str) -> Result<Module> {
        run_parser(input).context("Phase: parse").map_err(|err| {
            if let Some(cerr) = err.downcast_ref::<CompileError>() {
                match cerr {
                    CompileError::ParseError { position, .. } => {
                        let mut lines = input.lines();
                        let current_line_number = input[..*position].lines().count();
                        let prev_line = lines.nth(current_line_number - 2).unwrap();
                        let current_line = lines.next().unwrap();
                        let next_line = lines.next().unwrap();

                        let mut current_line_position = *position;
                        while &input[current_line_position - 1..current_line_position] != "\n"
                            && current_line_position > 0
                        {
                            current_line_position -= 1;
                        }

                        let current_line_width = position - current_line_position;

                        let message = format!(
                            "position: {}, line: {}, width: {}\n{}",
                            position,
                            current_line_number,
                            current_line_width,
                            vec![
                                format!("{}\t| {}", current_line_number - 1, prev_line),
                                format!("{}\t| {}", current_line_number, current_line),
                                format!(
                                    "{}\t| {}^",
                                    current_line_number,
                                    repeat(' ').take(current_line_width).collect::<String>()
                                ),
                                format!("{}\t| {}", current_line_number + 1, next_line),
                            ]
                            .join("\n")
                        );
                        err.context(message)
                    }
                }
            } else {
                err
            }
        })
    }

    pub fn typecheck(&self, module: &mut Module) -> Result<TypeChecker> {
        let mut checker = TypeChecker::new(stdlib(), stdlib_methods());
        checker.module(module).context("Phase: typecheck")?;

        Ok(checker)
    }

    pub fn exec(&self, input: &str) -> Result<DataValue> {
        let mut module = self.parse(input)?;
        let checker = self.typecheck(&mut module)?;

        let mut eval = Evaluator::new(checker.structs, checker.functions, checker.methods);
        eval.eval_module(module).context("Phase: eval")
    }
}
