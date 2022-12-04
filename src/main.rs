mod ast;
mod compiler;
mod generator;
mod lexer;
mod parser;
mod runtime;

use crate::{compiler::Compiler, runtime::Runtime};

fn main() -> anyhow::Result<()> {
    let mut compiler = Compiler::new();
    let mut runtime = Runtime::new();
    let wat = compiler.compile(
        r#"
fun main(): i32 {
let x: i32 = 10;
return x + 1;
}
"#,
    )?;
    println!("{}", wat);

    let result = runtime.run(&wat)?;
    println!("{:?}", result);

    Ok(())
}
