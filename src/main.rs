use std::{collections::HashMap, io::Read};

use anyhow::Result;

use crate::{
    code_gen::gen_code,
    parser::run_parser,
    runtime::{execute, FFIFunction},
    stdlib::create_ffi_table,
    typechecker::typecheck,
    vm::{HeapData, StackData},
};

mod ast;
mod code_gen;
mod compiler;
mod lexer;
mod parser;
mod runtime;
mod stdlib;
mod typechecker;
mod vm;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let (ffi_table, ffi_functions) = create_ffi_table();

    let mut module = run_parser(&buffer)?;
    typecheck(&mut module)?;

    let (program, static_area) = gen_code(module, ffi_table)?;
    println!("{:?}", execute(program, static_area, ffi_functions));

    Ok(())
}
