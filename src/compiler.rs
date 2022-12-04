use anyhow::{Context, Result};

use crate::{generator::Generator, lexer::Lexer, parser::Parser};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile(&mut self, input: &str) -> Result<String> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut generator = Generator::new();

        lexer.run(input).context("lexer phase")?;
        let mut ast = parser.run(lexer.tokens).context("parser phase")?;
        generator.run(&mut ast).context("generator phase")?;

        Ok(generator.writer)
    }
}
