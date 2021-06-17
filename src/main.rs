use std::io::Read;

// use crate::parser::run_parser;

mod ast;
mod lexer;
mod parser;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer)?;

    //println!("{:?}", run_parser(&buffer));

    Ok(())
}
