use std::{collections::HashMap, io::Read};

use anyhow::Result;
use runtime::FFITable;

use crate::{
    parser::run_parser,
    runtime::{interpret, DataType},
};

mod ast;
mod code_gen;
mod lexer;
mod parser;
mod runtime;

pub fn create_ffi_table() -> FFITable {
    let mut ffi_table: FFITable = HashMap::new();
    ffi_table.insert(
        "_add".to_string(),
        Box::new(|vs: Vec<DataType>| match (&vs[0], &vs[1]) {
            (DataType::Int(x), DataType::Int(y)) => DataType::Int(x + y),
            _ => todo!(),
        }),
    );
    ffi_table.insert(
        "_minus".to_string(),
        Box::new(|vs: Vec<DataType>| match (&vs[0], &vs[1]) {
            (DataType::Int(x), DataType::Int(y)) => DataType::Int(x - y),
            _ => todo!(),
        }),
    );

    ffi_table
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer)?;

    println!("{:?}", interpret(create_ffi_table(), run_parser(&buffer)?));

    Ok(())
}
