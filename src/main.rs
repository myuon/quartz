use std::{collections::HashMap, io::Read};

use anyhow::Result;

use crate::{
    code_gen::gen_code,
    parser::run_parser,
    runtime::{execute, DataType, FFIFunction},
};

mod ast;
mod code_gen;
mod lexer;
mod parser;
mod runtime;

pub fn create_ffi_table() -> (HashMap<String, usize>, Vec<FFIFunction>) {
    let mut ffi_table = HashMap::<String, FFIFunction>::new();
    ffi_table.insert(
        "_add".to_string(),
        Box::new(|mut vs: Vec<DataType>| {
            let x = vs.pop().unwrap();
            let y = vs.pop().unwrap();

            match (x, y) {
                (DataType::Int(x), DataType::Int(y)) => vs.push(DataType::Int(x + y)),
                _ => todo!(),
            }

            vs
        }),
    );

    let enumerated = ffi_table.into_iter().enumerate().collect::<Vec<_>>();

    let variables = enumerated
        .iter()
        .map(|(i, (k, _))| (k.clone(), *i))
        .collect::<HashMap<_, _>>();
    let table = enumerated.into_iter().map(|(_, (_, v))| v).collect();

    (variables, table)
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let (ffi_table, ffi_functions) = create_ffi_table();

    println!(
        "{:?}",
        execute(gen_code(run_parser(&buffer)?, ffi_table)?, ffi_functions)
    );

    Ok(())
}
