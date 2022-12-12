use std::{collections::HashSet, io::Read};

use anyhow::{Context, Result};
use thiserror::{Error, __private::PathAsDisplay};

use crate::{
    ast::Module,
    generator::Generator,
    ir_code_gen::IrCodeGenerator,
    lexer::Lexer,
    parser::Parser,
    typecheck::TypeChecker,
    util::{ident::Ident, path::Path},
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

    fn compile_(&mut self, cwd: &str, input: &str) -> Result<String> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut typechecker = TypeChecker::new();
        let mut generator = Generator::new();
        let mut ir_code_generator = IrCodeGenerator::new();

        let mut loaded_modules = vec![];

        lexer.run(input).context("lexer phase")?;
        loaded_modules.push(parser.run(lexer.tokens).context("parser phase")?.0);

        let mut visited = HashSet::new();

        parser.imports.push(Path::new(vec![
            Ident("quartz".to_string()),
            Ident("std".to_string()),
        ]));

        while let Some(path) = parser.imports.pop() {
            if visited.contains(&path) {
                continue;
            }

            let mut lexer = Lexer::new();
            let mut parser = Parser::new();

            let file_path = std::path::Path::new(cwd)
                .join(
                    path.0
                        .iter()
                        .map(|ident| ident.as_str())
                        .collect::<Vec<&str>>()
                        .join("/"),
                )
                .with_extension("qz");
            let mut file = std::fs::File::open(&file_path)
                .context(format!("opening file {}", file_path.as_display()))?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).context("reading file")?;

            lexer.run(&buffer).context("lexer phase")?;
            loaded_modules.push(parser.run(lexer.tokens).context("parser phase")?.0);

            visited.insert(path);
        }

        let mut module = Module(loaded_modules.concat());

        typechecker.run(&mut module).context("typechecker phase")?;

        ir_code_generator.set_types(typechecker.types.clone());
        let mut ir = ir_code_generator
            .run(&mut module)
            .context("ir code generator phase")?;

        generator.set_globals(typechecker.globals.keys().into_iter().cloned().collect());
        generator.set_types(typechecker.types);
        generator.run(&mut ir).context("generator phase")?;

        Ok(generator.writer.buffer)
    }

    pub fn compile(&mut self, cwd: &str, input: &str) -> Result<String> {
        let input = input.to_string();

        self.compile_(cwd, &input).map_err(|error| {
            if let Some(source) = error.downcast_ref::<ErrorInSource>() {
                let start = source.start;
                let end = source.end;

                let (start_line_number, start_column_index) = find_position(&input, start);
                let start_line = input.lines().nth(start_line_number).unwrap();

                let (end_line_number, end_column_index) = find_position(&input, end);

                let line_number_gutter = format!("{}: ", start_line_number);

                error.context(format!(
                    "Error at (line.{}:{}) to (line.{}:{})\n{}{}\n{}{}",
                    start_line_number,
                    start_column_index,
                    end_line_number,
                    end_column_index,
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

    (
        line_number,
        if position > count {
            position - count
        } else {
            0
        },
    )
}
