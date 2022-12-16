use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

use anyhow::{Context, Result};
use thiserror::{Error, __private::PathAsDisplay};

use crate::{
    ast::{Decl, Module},
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
    pub path: Option<Path>,
    pub start: usize,
    pub end: usize,
}

struct SourceLoader {
    loaded: HashMap<Path, LoadedModule>,
}

struct LoadedModule {
    source: String,
    module: Module,
}

impl SourceLoader {
    pub fn new() -> SourceLoader {
        SourceLoader {
            loaded: HashMap::new(),
        }
    }

    pub fn load_module(&mut self, cwd: &str, path: Path) -> Result<()> {
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

        self.loaded.insert(
            path.clone(),
            LoadedModule {
                source: buffer.clone(),
                module: Module(vec![]),
            },
        );

        let mut lexer = Lexer::new();
        let mut parser = Parser::new();

        lexer.run(&buffer, path.clone()).context("lexer phase")?;
        let module = parser
            .run(lexer.tokens, path.clone())
            .context("parser phase")?;

        self.loaded.insert(
            path,
            LoadedModule {
                source: buffer.clone(),
                module,
            },
        );

        Ok(())
    }

    pub fn get(&self, path: &Path) -> Option<&LoadedModule> {
        self.loaded.get(path)
    }
}

pub struct Compiler {
    loader: SourceLoader,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            loader: SourceLoader::new(),
        }
    }

    fn compile_(&mut self, cwd: &str, input: &str) -> Result<String> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut typechecker = TypeChecker::new();
        let mut generator = Generator::new();
        let mut ir_code_generator = IrCodeGenerator::new();

        let main_path = Path::ident(Ident("main".to_string()));
        lexer.run(input, main_path.clone()).context("lexer phase")?;
        let main = parser
            .run(lexer.tokens, main_path.clone())
            .context("parser phase")?;

        self.loader.loaded.insert(
            main_path.clone(),
            LoadedModule {
                source: input.to_string(),
                module: main.clone(),
            },
        );

        let mut visited = HashSet::new();

        parser.imports.push(Path::new(vec![
            Ident("quartz".to_string()),
            Ident("std".to_string()),
        ]));

        while let Some(path) = parser.imports.pop() {
            if visited.contains(&path) {
                continue;
            }

            self.loader.load_module(cwd, path.clone())?;

            visited.insert(path);
        }

        let mut module = Module(
            self.loader
                .loaded
                .iter()
                .map(|(k, v)| {
                    Decl::Module(
                        Ident(
                            k.0.iter()
                                .map(|v| v.as_str())
                                .collect::<Vec<_>>()
                                .join("_")
                                .to_string(),
                        ),
                        v.module.clone(),
                    )
                })
                .collect::<Vec<_>>(),
        );
        module.0.push(Decl::Module(Ident("main".to_string()), main));

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
                let input = if let Some(path) = &source.path {
                    self.loader.get(path).unwrap().source.clone()
                } else {
                    input
                };

                let start = source.start;
                let end = source.end;
                let source_path = source.path.clone();

                let (start_line_number, start_column_index) = find_position(&input, start);
                let start_line = input.lines().nth(start_line_number).unwrap();

                let (end_line_number, end_column_index) = find_position(&input, end);

                let line_number_gutter = format!("{}: ", start_line_number);

                error.context(format!(
                    "Error at {}, (line.{}:{}) to (line.{}:{})\n{}{}\n{}{}",
                    source_path
                        .unwrap_or(Path::ident(Ident("main".to_string())))
                        .as_str(),
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
