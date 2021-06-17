use std::io::Read;

use crate::parser::run_parser;

mod ast;
mod code_gen;
mod lexer;
mod parser;
mod runtime;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer)?;

    println!("{:?}", run_parser(&buffer));

    Ok(())
}
