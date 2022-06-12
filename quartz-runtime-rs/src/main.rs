use std::io::Read;

use anyhow::Result;
use quartz_core::compiler::Compiler;
use runtime::Runtime;

mod freelist;
mod runtime;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer).unwrap();

    let mut compiler = Compiler::new();
    let code = compiler.compile(&buffer)?;
    println!(
        "{:?}",
        Runtime::new(code.clone(), compiler.code_generation.globals())
            .run()
            .unwrap()
    );

    Ok(())
}
