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

use anyhow::Result;
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
    Compile {
        #[clap(long)]
        stdin: bool,

        file: Option<String>,
    },
    #[clap(name = "run", about = "Run a file")]
    Run {
        #[clap(long)]
        stdin: bool,

        file: Option<String>,
    },
    #[clap(name = "run-wat", about = "Run a wat file")]
    RunWat {
        #[clap(long)]
        stdin: bool,

        file: Option<String>,
    },
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn main() -> Result<()> {
    let mut compiler = Compiler::new();
    let mut runtime = Runtime::new();

    let cli = Cli::parse();
    match cli.subcmd {
        SubCommand::Compile { stdin, file } => {
            compile(&mut compiler, stdin, file)?;
        }
        SubCommand::Run { stdin, file } => {
            let wat = compile(&mut compiler, stdin, file)?;
            let result = runtime.run(&wat)?;
            if !result.is_empty() {
                for r in result.iter() {
                    println!("{}", r.to_string());
                }
            }
        }
        SubCommand::RunWat { stdin, file } => {
            let wat = if stdin {
                read_from_stdin()
            } else {
                let path = file.ok_or(anyhow::anyhow!("No file specified"))?;
                let mut file = std::fs::File::open(path)?;
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;

                buffer
            };

            let result = runtime.run(&wat)?;
            if !result.is_empty() {
                println!("{:?}", result);
            }
        }
    }

    Ok(())
}

fn compile(compiler: &mut Compiler, stdin: bool, file: Option<String>) -> Result<String> {
    let cwd = std::env::current_dir()?.to_str().unwrap().to_string();
    let input = if stdin {
        read_from_stdin()
    } else {
        let path = file.ok_or(anyhow::anyhow!("No file specified"))?;
        let mut file = std::fs::File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        buffer
    };
    let wat = compiler.compile(&cwd, &input)?;

    let file = std::fs::File::create("build/build.wat")?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(wat.as_bytes())?;

    let file = std::fs::File::create("build/build.wasm")?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(&wat::parse_str(&wat)?)?;

    Ok(wat)
}
