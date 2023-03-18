use std::{collections::HashSet, io::Read};

use anyhow::{Context, Result};
use serde::Serialize;
use thiserror::{Error, __private::PathAsDisplay};

use crate::{
    ast::{Decl, Module},
    generator::Generator,
    ir::IrTerm,
    ir_code_gen::IrCodeGenerator,
    lexer::Lexer,
    parser::Parser,
    typecheck::TypeChecker,
    util::{ident::Ident, path::Path},
};

pub const MODE_ASSERT_PTR: bool = true;
pub const MODE_TYPE_REP: bool = true;
pub const MODE_OPTIMIZE_ARITH_OPS_IN_CODE_GEN: bool = true;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SourcePosition {
    pub path: Path,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Error)]
#[error("Found error in span ({start:?},{end:?})")]
pub struct ErrorInSource {
    pub path: Option<Path>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Serialize)]
pub struct ElaboratedError {
    pub source_path: Option<Vec<String>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub message: String,
}

#[derive(Debug, Clone)]
struct SourceLoader {
    loaded: Vec<LoadedModule>,
}

#[derive(Debug, Clone)]
struct LoadedModule {
    path: Path,
    source: String,
    module: Module,
}

impl SourceLoader {
    pub fn new() -> SourceLoader {
        SourceLoader { loaded: vec![] }
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

        self.loaded.push(LoadedModule {
            path: path.clone(),
            source: buffer.clone(),
            module: Module(vec![]),
        });

        let mut lexer = Lexer::new();
        let mut parser = Parser::new();

        lexer.run(&buffer, path.clone()).context("lexer phase")?;
        let module = parser
            .run(lexer.tokens, path.clone())
            .context("parser phase")?;

        self.loaded.push(LoadedModule {
            path,
            source: buffer.clone(),
            module,
        });

        Ok(())
    }

    pub fn matches(&self, path: &Path) -> Option<&LoadedModule> {
        self.loaded.iter().find(|v| path.starts_with(&v.path))
    }
}

pub struct Compiler {
    loader: SourceLoader,
    pub ir: Option<IrTerm>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            loader: SourceLoader::new(),
            ir: None,
        }
    }

    fn check_(&mut self, cwd: &str, input: &str) -> Result<()> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut typechecker = TypeChecker::new();

        let main_path = Path::ident(Ident("main".to_string()));
        lexer.run(input, main_path.clone()).context("lexer phase")?;
        let main = parser
            .run(lexer.tokens, main_path.clone())
            .context("parser phase")?;

        self.loader.loaded.push(LoadedModule {
            path: main_path.clone(),
            source: input.to_string(),
            module: main.clone(),
        });

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
                .map(|v| {
                    // HACK: Add import quartz::std to the top of the file
                    let mut decls = vec![Decl::Import(v.path.clone())];
                    decls.extend(v.module.0.clone());

                    Decl::Module(v.path.clone(), Module(decls))
                })
                .collect::<Vec<_>>(),
        );

        typechecker.run(&mut module).context("typechecker phase")?;

        Ok(())
    }

    pub fn check(&mut self, cwd: &str, input: &str) -> Vec<ElaboratedError> {
        let input = input.to_string();

        let Err(error) = self.check_(cwd, &input) else {
            return vec![]
        };

        if let Some(source) = error.downcast_ref::<ErrorInSource>() {
            let input = if let Some(path) = &source.path {
                let Some(loaded) = self.loader.matches(path) else {
                        return vec![]
                    };

                loaded.source.clone()
            } else {
                input
            };

            let start = source.start;
            let end = source.end;
            let source_path = source.path.clone();

            let (start_line_number, start_column_index) = find_position(&input, start);
            let (end_line_number, end_column_index) = find_position(&input, end);

            vec![ElaboratedError {
                source_path: source_path.map(|path| path.0.into_iter().map(|t| t.0).collect()),
                start: (start_line_number, start_column_index),
                end: (end_line_number, end_column_index),
                message: format!("{:?}", error),
            }]
        } else {
            vec![ElaboratedError {
                start: (0, 0),
                end: (0, 0),
                source_path: None,
                message: format!("Unknown error: {}", error),
            }]
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

        self.loader.loaded.push(LoadedModule {
            path: main_path.clone(),
            source: input.to_string(),
            module: main.clone(),
        });

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
                .map(|v| {
                    // HACK: Add import quartz::std to the top of the file
                    let mut decls = vec![Decl::Import(v.path.clone())];
                    decls.extend(v.module.0.clone());

                    Decl::Module(v.path.clone(), Module(decls))
                })
                .collect::<Vec<_>>(),
        );

        typechecker.run(&mut module).context("typechecker phase")?;

        ir_code_generator.set_types(typechecker.types.clone());
        let mut ir = ir_code_generator
            .run(&mut module)
            .context("ir code generator phase")?;

        generator.set_cwd(Path::ident(Ident("main".to_string())));
        generator.set_globals(typechecker.globals.keys().into_iter().cloned().collect());
        generator.set_types(typechecker.types);
        generator.set_strings(ir_code_generator.strings);
        generator.run(&mut ir).context("generator phase")?;

        self.ir = Some(ir);

        Ok(generator.writer.buffer)
    }

    pub fn compile(&mut self, cwd: &str, input: &str) -> Result<String> {
        let input = input.to_string();

        self.compile_(cwd, &input).map_err(|error| {
            if let Some(source) = error.downcast_ref::<ErrorInSource>() {
                let input = if let Some(path) = &source.path {
                    let Some(loaded) = self.loader.matches(path) else {
                        return error
                    };

                    loaded.source.clone()
                } else {
                    input
                };

                let start = source.start;
                let end = source.end;
                let source_path = source.path.clone();

                let (start_line_number, start_column_index) = find_position(&input, start);
                let (end_line_number, end_column_index) = find_position(&input, end);

                let mut result = String::new();

                for (index, line) in input.lines().collect::<Vec<_>>()
                    [start_line_number..end_line_number]
                    .into_iter()
                    .enumerate()
                {
                    let line_number_gutter = format!("{}: ", start_line_number + index + 1);
                    result += &format!(
                        "{}{}\n{}\n",
                        line_number_gutter,
                        line,
                        if index == 0 {
                            format!(
                                "{}{}",
                                " ".repeat(line_number_gutter.len() + start_column_index),
                                "^".repeat(line.len() - start_column_index)
                            )
                        } else {
                            format!(
                                "{}{}",
                                " ".repeat(line_number_gutter.len()),
                                "^".repeat(line.len())
                            )
                        }
                    );
                }
                let line_number_gutter = format!("{}: ", end_line_number + 1);
                result += &format!(
                    "{}{}\n{}\n",
                    line_number_gutter,
                    input.lines().nth(end_line_number).unwrap(),
                    format!(
                        "{}{}",
                        " ".repeat(line_number_gutter.len()),
                        "^".repeat(end_column_index)
                    )
                );

                error.context(format!(
                    "Error at {}, (line.{}:{}) to (line.{}:{})\n{}",
                    source_path
                        .unwrap_or(Path::ident(Ident("main".to_string())))
                        .as_str(),
                    start_line_number + 1,
                    start_column_index,
                    end_line_number + 1,
                    end_column_index,
                    result,
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
