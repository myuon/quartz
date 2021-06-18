use std::{collections::HashMap, io::Read};

use anyhow::Result;

use crate::{parser::run_parser, runtime::interpret};

mod ast;
mod code_gen;
mod lexer;
mod parser;
mod runtime;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer)?;

    println!("{:?}", interpret(HashMap::new(), run_parser(&buffer)?));

    Ok(())
}
