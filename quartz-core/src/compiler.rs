use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    iter::repeat,
    path::PathBuf,
};

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

    let mut current_line_position = position;
    while current_line_position > 0
        && &input[current_line_position - 1..current_line_position] != "\n"
    {
        current_line_position -= 1;
    }

    let current_line_width = position - current_line_position;

    let mut end_line_position = end;
    while end_line_position < input.len()
        && &input[end_line_position..end_line_position + 1] != "\n"
    {
        end_line_position += 1;
    }

    let mut code_lines = vec![];
    if current_line_number >= 2 {
        let prev_line = lines.nth(current_line_number - 2).unwrap();
        code_lines.push(format!("{}\t| {}", current_line_number - 1, prev_line));
    }

    let range_lines_count = {
        let mut result = 0;
        let mut position = start;
        while position < end {
            if &input[position..position + 1] == "\n" {
                result += 1;
            }

            position += 1;
        }

        result += 1;

        result as usize
    };

    for i in 0..range_lines_count {
        let current_line = lines.next().unwrap();

        code_lines.push(format!("{}\t| {}", current_line_number + i, current_line));
        if range_lines_count == 1 {
            code_lines.push(format!(
                "{}\t| {}{}",
                current_line_number + i,
                repeat(' ').take(current_line_width).collect::<String>(),
                repeat('^').take(end - start).collect::<String>(),
            ));
        } else if i == 0 {
            code_lines.push(format!(
                "{}\t| {}{}",
                current_line_number + i,
                repeat(' ').take(current_line_width).collect::<String>(),
                repeat('^')
                    .take(current_line.len() - current_line_width)
                    .collect::<String>(),
            ));
        } else {
            code_lines.push(format!(
                "{}\t| {}",
                current_line_number + i,
                repeat('^').take(current_line.len()).collect::<String>(),
            ));
        }
    }

    code_lines.push(format!(
        "{}\t| {}",
        current_line_number + range_lines_count,
        lines.next().unwrap_or("")
    ));

    format!(
        "position: {}, line: {}, width: {}\n{}",
        position,
        current_line_number,
        current_line_width,
        code_lines.join("\n")
    )
}

pub struct Compiler<'s> {
    pub typechecker: TypeChecker<'s>,
    pub vm_code_generation: VmGenerator,
    pub ir_result: Option<IrElement>,
    pub ir_source_map: HashMap<usize, String>,
}

impl Compiler<'_> {
    pub fn new() -> Compiler<'static> {
        Compiler {
            typechecker: TypeChecker::new(builtin(), Structs(HashMap::new()), ""),
            vm_code_generation: VmGenerator::new(),
            ir_result: None,
            ir_source_map: HashMap::new(),
        }
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

    pub fn typecheck(&mut self, modules: &mut Vec<Module>) -> Result<TypeChecker> {
        self.typechecker
            .modules(modules)
            .context("Phase: typecheck")?;

        Ok(self.typechecker.clone())
    }

    pub fn compile_ir<'s>(&mut self, input: &'s str, entrypoint: String) -> Result<IrElement> {
        self.compile_ir_nostd(&input, entrypoint, Some("std".to_string()))
    }

    pub fn compile_ir_nostd<'s>(
        &'s mut self,
        input: &'s str,
        entrypoint: String,
        preload_path: Option<String>,
    ) -> Result<IrElement> {
        let mut typechecker = TypeChecker::new(
            self.typechecker.variables.clone(),
            self.typechecker.structs.clone(),
            &input,
        );

        let mut modules = vec![];
        let mut visited = HashSet::new();
        let mut stack = vec!["main".to_string()];
        let mut buffer = String::new();
        if let Some(path) = preload_path {
            stack.push(path.clone());
            visited.insert(path);
        }

        while let Some(path) = stack.pop() {
            let current = if path == "main" {
                input
            } else {
                let mut d = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
                d.push(format!("{}.qz", path.clone()));

                let mut f = File::open(format!("{}", d.display()))
                    .context(format!("Load file: {}", d.display()))?;
                f.read_to_string(&mut buffer)?;

                &buffer
            };
            let module = self.parse(&current).context("parse phase")?;
            for path in module.imports.clone() {
                if !visited.contains(&path) {
                    stack.push(path.clone());
                    visited.insert(path);
                }
            }

            modules.push(module);
            info!("parsed module: {}", path);
        }

        typechecker.set_entrypoint(entrypoint);
        typechecker
            .modules(&mut modules)
            .context("typecheck phase")?;
        info!("typecheck");

        let mut ir_code_generation = IrGenerator::new(&input);
        ir_code_generation.context(typechecker.structs.clone());

        let code = ir_code_generation.generate(&modules)?;
        info!("ir generated");

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
        let code = self.vm_code_generation.generate(ir)?;
        self.ir_source_map = self.vm_code_generation.source_map.clone();

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
        let code = self.vm_code_generation.generate(ir)?;
        self.ir_source_map = self.vm_code_generation.source_map.clone();

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
