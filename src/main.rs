use std::io::Read;

use anyhow::Result;
use compiler::Compiler;

mod ast;
mod compiler;
mod eval;
mod lexer;
mod parser;
mod stdlib;
mod typechecker;
mod vm;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let compiler = Compiler::new();
    compiler.exec(&buffer)?;

    Ok(())
}
