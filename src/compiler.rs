use anyhow::{Context, Result};
use thiserror::Error;

use crate::{
    generator::Generator, ir_code_gen::IrCodeGenerator, lexer::Lexer, parser::Parser,
    typecheck::TypeChecker,
};

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
        let mut ir_code_generator = IrCodeGenerator::new();

        lexer.run(input).context("lexer phase")?;
        let mut ast = parser.run(lexer.tokens).context("parser phase")?;
        typechecker.run(&mut ast).context("typechecker phase")?;

        ir_code_generator.set_types(typechecker.types.clone());
        let mut ir = ir_code_generator
            .run(&mut ast)
            .context("ir code generator phase")?;

        generator.set_globals(typechecker.globals.keys().into_iter().cloned().collect());
        generator.set_types(typechecker.types);
        generator.run(&mut ir).context("generator phase")?;

        Ok(generator.writer.buffer)
    }

    pub fn compile(&mut self, input: &str) -> Result<String> {
        self.compile_(input).map_err(|error| {
            if let Some(source) = error.downcast_ref::<ErrorInSource>() {
                let start = source.start;
                let end = source.end;

                let (start_line_number, start_column_index) = find_position(input, start);
                let start_line = input.lines().nth(start_line_number).unwrap();

                let line_number_gutter = format!("{}: ", start_line_number);

                error.context(format!(
                    "\n{}{}\n{}{}",
                    line_number_gutter,
                    start_line,
                    " ".repeat(line_number_gutter.len() + start_column_index),
                    "^".repeat(end - start)
                ))
            } else {
                error
            }
        })
    }
}

fn find_position(input: &str, position: usize) -> (usize, usize) {
    let mut line_number = 0;
    let mut count = 0;
    for line in input.lines() {
        if count + line.len() > position {
            break;
        }

        line_number += 1;
        count += line.len() + 1;
    }

    (line_number, position - count)
}
