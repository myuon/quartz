use anyhow::{Context, Result};
use thiserror::Error;

use crate::{generator::Generator, lexer::Lexer, parser::Parser, typecheck::TypeChecker};

#[derive(Debug, Error)]
#[error("Found error in span ({start:?},{end:?})")]
pub struct ErrorInSource {
    pub start: usize,
    pub end: usize,
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    fn compile_(&mut self, input: &str) -> Result<String> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut typechecker = TypeChecker::new();
        let mut generator = Generator::new();

        lexer.run(input).context("lexer phase")?;
        let mut ast = parser.run(lexer.tokens).context("parser phase")?;
        typechecker.run(&mut ast).context("typechecker phase")?;

        generator.run(&mut ast).context("generator phase")?;

        Ok(generator.writer.buffer)
    }

    pub fn compile(&mut self, input: &str) -> Result<String> {
        self.compile_(input).map_err(|error| {
            if let Some(source) = error.downcast_ref::<ErrorInSource>() {
                let start = source.start;
                let end = source.end;
                error.context(format!(
                    "\n{}",
                    input[start..end]
                        .lines()
                        .enumerate()
                        .map(|(i, line)| format!("{}: {}", i + 1, line))
                        .collect::<Vec<String>>()
                        .join("\n")
                ))
            } else {
                error
            }
        })
    }
}
