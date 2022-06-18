use std::{collections::HashMap, fs::File, io::Read, iter::repeat, path::PathBuf};

use anyhow::{Context, Error, Result};
use thiserror::Error as ThisError;

use crate::{
    ast::{Methods, Module, Structs},
    builtin::builtin,
    code_generation::CodeGeneration,
    ir::IrElement,
    ir_code_generation::IrGenerator,
    parser::run_parser,
    typechecker::TypeChecker,
    vm::QVMInstruction,
};

#[derive(ThisError, Debug)]
pub enum CompileError {
    #[error("parse error: {source:?}")]
    ParseError { source: Error, position: usize },
}

pub struct Compiler<'s> {
    pub typechecker: TypeChecker<'s>,
    pub code_generation: CodeGeneration,
    pub ir_code_generation: IrGenerator,
}

impl Compiler<'_> {
    pub fn new() -> Compiler<'static> {
        Compiler {
            typechecker: TypeChecker::new(
                builtin(),
                Structs(HashMap::new()),
                Methods(HashMap::new()),
                "",
            ),
            code_generation: CodeGeneration::new(),
            ir_code_generation: IrGenerator::new(),
        }
    }

    fn load_std(&self) -> Result<String> {
        let mut d = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        d.push("std.qz");

        let mut f = File::open(format!("{}", d.display()))?;
        let mut buffer = String::new();

        f.read_to_string(&mut buffer)?;

        Ok(buffer)
    }

    fn with_std(&self, input: &str) -> Result<String> {
        let mut std = self.load_std()?;
        std.push_str(input);
        Ok(std)
    }

    fn run_parser(&self, input: &str) -> Result<Module> {
        run_parser(&input).context("Phase: parse").map_err(|err| {
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

    pub fn parse(&self, input: &str) -> Result<Module> {
        self.run_parser(&input)
    }

    pub fn typecheck(&mut self, module: &mut Module) -> Result<TypeChecker> {
        self.typechecker
            .module(module)
            .context("Phase: typecheck")?;

        Ok(self.typechecker.clone())
    }

    pub fn compile<'s>(&mut self, input: &'s str) -> Result<Vec<QVMInstruction>> {
        let input = self.with_std(input)?;
        let mut typechecker = TypeChecker::new(
            self.typechecker.variables.clone(),
            self.typechecker.structs.clone(),
            self.typechecker.methods.clone(),
            &input,
        );

        let mut module = self.parse(&input).context("parse phase")?;

        typechecker
            .module(&mut module)
            .context("Phase: typecheck")?;

        self.code_generation.context(typechecker.structs.clone());

        let code = self.code_generation.generate(&module)?;

        self.typechecker = TypeChecker::new(
            self.typechecker.variables.clone(),
            self.typechecker.structs.clone(),
            self.typechecker.methods.clone(),
            "",
        );

        Ok(code)
    }

    pub fn compile_ir<'s>(&mut self, input: &'s str) -> Result<IrElement> {
        let input = self.with_std(input)?;

        self.compile_ir_nostd(&input)
    }

    pub fn compile_ir_nostd<'s>(&mut self, input: &'s str) -> Result<IrElement> {
        let mut typechecker = TypeChecker::new(
            self.typechecker.variables.clone(),
            self.typechecker.structs.clone(),
            self.typechecker.methods.clone(),
            &input,
        );

        let mut module = self.parse(&input).context("parse phase")?;

        typechecker.module(&mut module)?;

        self.ir_code_generation.context(typechecker.structs.clone());

        let code = self.ir_code_generation.generate(&module)?;

        self.typechecker = TypeChecker::new(
            self.typechecker.variables.clone(),
            self.typechecker.structs.clone(),
            self.typechecker.methods.clone(),
            "",
        );

        Ok(code)
    }
}
