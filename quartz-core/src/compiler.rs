use std::{collections::HashMap, fs::File, io::Read, iter::repeat, path::PathBuf};

use anyhow::{Context, Error, Result};
use log::info;
use thiserror::Error as ThisError;

use crate::{
    ast::{Module, Structs},
    builtin::builtin,
    ir::IrElement,
    ir_code_generation::IrGenerator,
    parser::run_parser,
    typechecker::TypeChecker,
    vm::QVMInstruction,
    vm_code_generation::VmGenerator,
};

#[derive(ThisError, Debug)]
pub enum CompileError {
    #[error("parse error: {source:?}")]
    ParseError { source: Error, position: usize },
}

pub fn specify_source_in_input(input: &str, start: usize, end: usize) -> String {
    let position = start;

    let mut lines = input.lines();
    let current_line_number = input[..position].lines().count();
    let prev_line = lines.nth(current_line_number - 2).unwrap();
    let current_line = lines.next().unwrap();
    let next_line = lines.next().unwrap_or(current_line);

    let mut current_line_position = position;
    while &input[current_line_position - 1..current_line_position] != "\n"
        && current_line_position > 0
    {
        current_line_position -= 1;
    }

    let current_line_width = position - current_line_position;

    format!(
        "position: {}, line: {}, width: {}\n{}",
        position,
        current_line_number,
        current_line_width,
        vec![
            format!("{}\t| {}", current_line_number - 1, prev_line),
            format!("{}\t| {}", current_line_number, current_line),
            format!(
                "{}\t| {}{}",
                current_line_number,
                repeat(' ').take(current_line_width).collect::<String>(),
                repeat('^').take(end - start).collect::<String>(),
            ),
            format!("{}\t| {}", current_line_number + 1, next_line),
        ]
        .join("\n")
    )
}

pub struct Compiler<'s> {
    pub typechecker: TypeChecker<'s>,
    pub ir_code_generation: IrGenerator,
    pub vm_code_generation: VmGenerator,
    pub ir_result: Option<IrElement>,
    pub ir_source_map: HashMap<usize, String>,
}

impl Compiler<'_> {
    pub fn new() -> Compiler<'static> {
        Compiler {
            typechecker: TypeChecker::new(builtin(), Structs(HashMap::new()), ""),
            ir_code_generation: IrGenerator::new(),
            vm_code_generation: VmGenerator::new(),
            ir_result: None,
            ir_source_map: HashMap::new(),
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
                        let message = specify_source_in_input(input, *position, *position + 1);
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

    pub fn compile_ir<'s>(&mut self, input: &'s str, entrypoint: String) -> Result<IrElement> {
        let input = self.with_std(input)?;

        self.compile_ir_nostd(&input, entrypoint)
    }

    pub fn compile_ir_nostd<'s>(
        &mut self,
        input: &'s str,
        entrypoint: String,
    ) -> Result<IrElement> {
        let mut typechecker = TypeChecker::new(
            self.typechecker.variables.clone(),
            self.typechecker.structs.clone(),
            &input,
        );

        let mut module = self.parse(&input).context("parse phase")?;

        typechecker.set_entrypoint(entrypoint);
        typechecker.module(&mut module).context("typecheck phase")?;

        self.ir_code_generation.context(typechecker.structs.clone());

        let code = self.ir_code_generation.generate(&module)?;

        self.typechecker = TypeChecker::new(
            self.typechecker.variables.clone(),
            self.typechecker.structs.clone(),
            "",
        );

        Ok(code)
    }

    pub fn compile<'s>(
        &mut self,
        input: &'s str,
        entrypoint: String,
    ) -> Result<Vec<QVMInstruction>> {
        let ir = self.compile_ir(input, entrypoint.clone())?;
        self.ir_result = Some(ir.clone());
        self.vm_code_generation.set_entrypoint(entrypoint);
        let (code, source_map) = self.vm_code_generation.generate(ir)?;
        self.ir_source_map = source_map;

        self.typechecker = TypeChecker::new(
            self.typechecker.variables.clone(),
            self.typechecker.structs.clone(),
            "",
        );

        Ok(code)
    }

    pub fn compile_result<'s>(
        &mut self,
        input: &'s str,
        entrypoint: String,
    ) -> Result<Vec<QVMInstruction>> {
        let ir = self.compile_ir(input, entrypoint.clone())?;
        self.ir_result = Some(ir.clone());
        self.vm_code_generation.set_entrypoint(entrypoint);
        let (code, source_map) = self.vm_code_generation.generate(ir)?;
        self.ir_source_map = source_map;

        self.typechecker = TypeChecker::new(
            self.typechecker.variables.clone(),
            self.typechecker.structs.clone(),
            "",
        );

        Ok(code)
    }

    pub fn show_qasmv<'s>(&mut self, code: &'s [QVMInstruction]) -> String {
        let mut result = String::new();
        for (n, inst) in code.iter().enumerate() {
            if let Some(s) = self.ir_source_map.get(&n) {
                info!(";; {}", s);
                result += &format!(";; {}\n", s);
            }
            info!("{:04} {:?}", n, inst);
            result += &format!("{:04} {:?}\n", n, inst);
        }

        result
    }
}
