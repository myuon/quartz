use anyhow::Result;
use log::info;
use quartz_core::compiler::Compiler;
use runtime::Runtime;
use std::{
    env::{self, args},
    io::Read,
};

mod freelist;
mod runtime;

fn main() -> Result<()> {
    simplelog::TermLogger::init(
        if env::var("DEBUG") == Ok("true".to_string()) {
            simplelog::LevelFilter::Debug
        } else {
            simplelog::LevelFilter::Info
        },
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )?;

    let command = args().nth(1);
    if command == Some("test".to_string()) {
        let mut compiler = Compiler::new();
        let code = compiler.compile_via_ir("", "test")?;
        info!("{}", compiler.ir_result.unwrap().show());
        for (n, inst) in code.iter().enumerate() {
            info!("{:04} {:?}", n, inst);
        }

        Runtime::new(code.clone(), compiler.code_generation.globals()).run()?;
    } else if command == Some("compile_test".to_string()) {
        let mut compiler = Compiler::new();
        let code = compiler.compile_via_ir("", "test")?;
        info!("{}", compiler.ir_result.unwrap().show());
        for (n, inst) in code.iter().enumerate() {
            info!("{:04} {:?}", n, inst);
        }
    } else if command == Some("compile".to_string()) {
        let mut buffer = String::new();
        let mut stdin = std::io::stdin();
        stdin.read_to_string(&mut buffer).unwrap();

        let mut compiler = Compiler::new();
        let code = compiler.compile_via_ir(&buffer, "main")?;
        info!("{}", compiler.ir_result.unwrap().show());
        for (n, inst) in code.iter().enumerate() {
            info!("{:04} {:?}", n, inst);
        }
    } else {
        let mut buffer = String::new();
        let mut stdin = std::io::stdin();
        stdin.read_to_string(&mut buffer).unwrap();

        let mut compiler = Compiler::new();
        let code = compiler.compile_via_ir(&buffer, "main")?;

        Runtime::new(code.clone(), compiler.code_generation.globals()).run()?;
    }

    Ok(())
}
