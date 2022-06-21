use std::{env::args, io::Read};

use anyhow::Result;
use quartz_core::compiler::Compiler;
use runtime::Runtime;

mod freelist;
mod runtime;

fn main() -> Result<()> {
    let command = args().nth(1);
    if command == Some("test".to_string()) {
        let mut compiler = Compiler::new();
        let code = compiler.compile_via_ir("", "test")?;
        println!("{}", compiler.ir_result.unwrap().show());
        for (n, inst) in code.iter().enumerate() {
            println!("{:04} {:?}", n, inst);
        }

        println!(
            "{:?}",
            Runtime::new(code.clone(), compiler.code_generation.globals())
                .run()
                .unwrap()
        );
    } else if command == Some("compile_test".to_string()) {
        let mut compiler = Compiler::new();
        let code = compiler.compile_via_ir("", "test")?;
        println!("{}", compiler.ir_result.unwrap().show());
        for (n, inst) in code.iter().enumerate() {
            println!("{:04} {:?}", n, inst);
        }
    } else if command == Some("compile".to_string()) {
        let mut buffer = String::new();
        let mut stdin = std::io::stdin();
        stdin.read_to_string(&mut buffer).unwrap();

        let mut compiler = Compiler::new();
        let code = compiler.compile_via_ir(&buffer, "main")?;
        println!("{}", compiler.ir_result.unwrap().show());
        for (n, inst) in code.iter().enumerate() {
            println!("{:04} {:?}", n, inst);
        }
    } else {
        let mut buffer = String::new();
        let mut stdin = std::io::stdin();
        stdin.read_to_string(&mut buffer).unwrap();

        let mut compiler = Compiler::new();
        let code = compiler.compile_via_ir(&buffer, "main")?;
        println!(
            "{:?}",
            Runtime::new(code.clone(), compiler.code_generation.globals())
                .run()
                .unwrap()
        );
    }

    Ok(())
}
