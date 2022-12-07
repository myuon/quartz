mod ast;
mod compiler;
mod generator;
mod ir;
mod ir_code_gen;
mod lexer;
mod parser;
mod runtime;
mod typecheck;
mod util;

use std::io::{Read, Write};

use clap::{Parser, Subcommand};

use crate::{compiler::Compiler, runtime::Runtime};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    #[clap(name = "compile", about = "Compile a file")]
    Compile,
    #[clap(name = "run", about = "Run a file")]
    Run,
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn main() -> anyhow::Result<()> {
    let mut compiler = Compiler::new();
    let mut runtime = Runtime::new();

    let cli = Cli::parse();
    match cli.subcmd {
        SubCommand::Compile => {
            let wat = compiler.compile(&read_from_stdin())?;
            println!("{}", wat);

            let file = std::fs::File::create("build/out.wat")?;
            let mut writer = std::io::BufWriter::new(file);
            writer.write_all(wat.as_bytes())?;
        }
        SubCommand::Run => {
            let wat = compiler.compile(&read_from_stdin())?;
            let result = runtime.run(&wat)?;
            println!("{:?}", result);
        }
    }

    Ok(())
}
