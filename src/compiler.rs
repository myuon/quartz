use std::{collections::HashSet, io::Read};

use anyhow::{bail, Context, Result};
use serde::Serialize;
use thiserror::{Error, __private::PathAsDisplay};

use crate::{
    ast::{Decl, Module},
    formatter::Formatter,
    generator::Generator,
    ir::IrTerm,
    ir_code_gen::IrCodeGenerator,
    lexer::{Lexeme, Lexer, Token},
    parser::Parser,
    typecheck::TypeChecker,
    util::{ident::Ident, path::Path, source::Source},
};

pub const MODE_TYPE_REP: bool = true;
pub const MODE_OPTIMIZE_ARITH_OPS_IN_CODE_GEN: bool = true;
pub const MODE_READABLE_WASM: bool = true;

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

#[derive(Debug, Clone)]
pub struct GoToDefOutput {
    pub module_name: String,
    pub start: (usize, usize),
    pub end: (usize, usize),
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
            .run(lexer.tokens, path.clone(), false)
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

    fn get_main_path() -> Path {
        Path::ident(Ident("main".to_string()))
    }

    pub fn run_lexer(input: &str, path: Path) -> Result<Vec<Token>> {
        let mut lexer = Lexer::new();

        lexer.run(input, path)?;

        Ok(lexer.tokens)
    }

    pub fn run_parser(input: &str, path: Path, skip_errors: bool) -> Result<Module> {
        Ok(Compiler::run_parser_with_comments(input, path, skip_errors)?.0)
    }

    pub fn run_parser_with_comments(
        input: &str,
        path: Path,
        skip_errors: bool,
    ) -> Result<(Module, Vec<Token>)> {
        let mut parser = Parser::new();
        let tokens = Compiler::run_lexer(input, path.clone())?;
        let (comments, no_comments): (Vec<_>, Vec<_>) = tokens
            .iter()
            .partition(|v| matches!(v.lexeme, Lexeme::Comment(_)));

        Ok((
            parser.run(
                no_comments.into_iter().cloned().collect::<Vec<_>>(),
                path,
                skip_errors,
            )?,
            comments.into_iter().cloned().collect::<Vec<_>>(),
        ))
    }

    pub fn parse(
        &mut self,
        cwd: &str,
        main_path: Path,
        input: &str,
        skip_errors: bool,
    ) -> Result<Module> {
        let mut parser = Parser::new();

        let main = Compiler::run_parser(input, main_path.clone(), skip_errors)?;

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

        for decl in &main.0 {
            match &decl.data {
                Decl::Import(path) => {
                    parser.imports.push(path.clone());
                }
                _ => {}
            }
        }

        while let Some(path) = parser.imports.pop() {
            if visited.contains(&path) {
                continue;
            }

            self.loader.load_module(cwd, path.clone())?;

            visited.insert(path);
        }

        let module = Module(
            self.loader
                .loaded
                .iter()
                .map(|v| {
                    // HACK: Add import quartz::std to the top of the file
                    let mut decls = vec![Source::unknown(Decl::Import(v.path.clone()))];
                    decls.extend(v.module.0.clone());

                    Source::unknown(Decl::Module(v.path.clone(), Module(decls)))
                })
                .collect::<Vec<_>>(),
        );

        Ok(module)
    }

    fn check_(&mut self, cwd: &str, main_path: Path, input: &str) -> Result<(TypeChecker, Module)> {
        let mut module = self.parse(cwd, main_path, input, false)?;
        let mut typechecker = TypeChecker::new();

        typechecker.run(&mut module).context("typechecker phase")?;

        Ok((typechecker, module))
    }

    pub fn check(&mut self, cwd: &str, main_path: Path, input: &str) -> Vec<ElaboratedError> {
        let input = input.to_string();

        let Err(error) = self.check_(cwd, main_path, &input) else {
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
                message: format!("Unknown error: {:?}", error),
            }]
        }
    }

    fn compile_(
        &mut self,
        cwd: &str,
        input: &str,
        entrypoint_name: Option<Ident>,
    ) -> Result<String> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut typechecker = TypeChecker::new();
        let mut generator = Generator::new();
        let mut ir_code_generator = IrCodeGenerator::new();

        let main_path = Path::ident(Ident("main".to_string()));
        lexer.run(input, main_path.clone()).context("lexer phase")?;
        let main = parser
            .run(lexer.tokens, main_path.clone(), false)
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
                    let mut decls = vec![Source::unknown(Decl::Import(v.path.clone()))];
                    decls.extend(v.module.0.clone());

                    Source::unknown(Decl::Module(v.path.clone(), Module(decls)))
                })
                .collect::<Vec<_>>(),
        );

        typechecker.run(&mut module).context("typechecker phase")?;

        ir_code_generator.set_types(typechecker.types.clone());
        let mut ir = ir_code_generator
            .run(&mut module)
            .context("ir code generator phase")?;

        let entrypoint = Path::new(
            vec![
                main_path.0,
                vec![entrypoint_name.unwrap_or(Ident("main".to_string()))],
            ]
            .concat(),
        );

        let used = ir_code_generator.check_used(entrypoint.clone());
        println!("{:?}", ir);

        generator.set_entrypoint(entrypoint);
        generator.set_globals(typechecker.globals.keys().into_iter().cloned().collect());
        generator.set_strings(
            ir_code_generator
                .strings
                .to_vec()
                .into_iter()
                .map(|p| p.0)
                .collect(),
        );
        generator.set_used(used.into_iter().map(|t| t.as_joined_str("_")).collect());
        generator
            .run(&mut ir, ir_code_generator.data_section_offset)
            .context("generator phase")?;

        self.ir = Some(ir);

        Ok(generator.writer.buffer)
    }

    pub fn compile(
        &mut self,
        cwd: &str,
        input: &str,
        entrypoint_name: Option<Ident>,
    ) -> Result<String> {
        let input = input.to_string();

        self.compile_(cwd, &input, entrypoint_name)
            .map_err(|error| {
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

    pub fn check_type(
        &mut self,
        cwd: &str,
        module_path: Path,
        input: &str,
        line: usize,
        column: usize,
    ) -> String {
        let Ok(mut module) = self.parse(cwd, module_path.clone(), input, false) else {
            return String::new();
        };
        let position = find_line_column_from_position(input, line, column);

        let mut typechecker = TypeChecker::new();

        let Ok(t) = typechecker.find_at_cursor(&mut module, module_path, position) else {
            return String::new();
        };

        if let Some(t) = t {
            format!("```quartz\n{}\n```", t.to_string())
        } else {
            String::new()
        }
    }

    pub fn completion(
        &mut self,
        cwd: &str,
        module_path: Path,
        input: &str,
        line: usize,
        column: usize,
        dot: bool,
    ) -> Vec<(String, String, String)> {
        let parse_result = self.parse(cwd, module_path.clone(), input, true);
        let Ok(mut module) = parse_result else {
            return vec![];
        };
        let position = find_line_column_from_position(input, line, column);

        let mut typechecker = TypeChecker::new();

        let Ok(t) = typechecker.completion(&mut module, module_path, position, dot) else {
            return vec![];
        };

        t.unwrap_or(vec![])
    }

    pub fn go_to_def(
        &mut self,
        cwd: &str,
        module_path: Path,
        input: &str,
        line: usize,
        column: usize,
    ) -> Result<GoToDefOutput> {
        let mut module = self.parse(cwd, module_path.clone(), input, false)?;
        let position = find_line_column_from_position(input, line, column);

        let mut typechecker = TypeChecker::new();

        let result = typechecker.find_definition(&mut module, module_path, position)?;

        if let Some((path, start, end)) = result {
            let loaded = self.loader.matches(&path).unwrap();

            Ok(GoToDefOutput {
                module_name: path.0.last().unwrap().0.clone(),
                start: find_position(&loaded.source, start),
                end: find_position(&loaded.source, end),
            })
        } else {
            bail!("No definition found")
        }
    }

    pub fn format(input: &str) -> Result<String> {
        let (module, comments) = Compiler::run_parser_with_comments(
            input,
            Path::ident(Ident("main".to_string())),
            true,
        )?;
        let mut formatter = Formatter::new(input, &comments, 0);

        Ok(formatter.format(module))
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

fn find_line_column_from_position(input: &str, line: usize, column: usize) -> usize {
    let mut start = 0;
    let mut end = input.len();

    while start < end {
        let mid = (start + end) / 2;
        let (line_number, column_index) = find_position(input, mid);

        if line_number == line && column_index == column {
            return mid;
        }

        if line_number < line || (line_number == line && column_index < column) {
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    start
}
