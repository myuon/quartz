use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use quartz_core::{compiler::Compiler, vm::QVMSource};
use runtime::Runtime;
use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

mod freelist;
mod runtime;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(name = "run", about = "Run a Quartz program")]
    Run,
    #[clap(name = "test", about = "Run a Quartz test program")]
    Test,
    #[clap(name = "compile", about = "Compile a Quartz program")]
    Compile {
        #[clap(short, long, value_parser, value_name = "FILE")]
        qasm_output: Option<PathBuf>,
    },
    #[clap(name = "test_compiler", about = "Run Quartz compiler tests")]
    TestCompiler,
    #[clap(name = "debug", about = "Debug a Quartz program with debugger json")]
    Debug { debugger_json: Option<PathBuf> },
}

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

    let cli = Cli::parse();
    match cli.command {
        Command::Run => {
            let entrypoint = env::var("ENTRYPOINT").ok().unwrap_or("main".to_string());

            let mut buffer = String::new();
            let mut stdin = std::io::stdin();
            stdin.read_to_string(&mut buffer).unwrap();

            let mut compiler = Compiler::new();
            let code = compiler.compile(&buffer, entrypoint)?;

            Runtime::new(code.clone(), compiler.vm_code_generation.globals()).run()?;
        }
        Command::Compile { qasm_output } => {
            let mut buffer = String::new();
            let mut stdin = std::io::stdin();
            stdin.read_to_string(&mut buffer).unwrap();

            let mut compiler = Compiler::new();
            let (code, source_map) = compiler.compile_result(&buffer, "main".to_string())?;
            info!("{}", compiler.ir_result.unwrap().show());
            for (n, inst) in code.iter().enumerate() {
                if let Some(s) = source_map.get(&n) {
                    info!(";; {}", s);
                }
                info!("{:04} {:?}", n, inst);
            }

            let mut file = File::create(qasm_output.unwrap_or("./out.qasm".into())).unwrap();
            file.write_all(
                &QVMSource::new(code)
                    .into_string()
                    .bytes()
                    .collect::<Vec<_>>(),
            )
            .unwrap();
        }
        Command::Test => {
            let entrypoint = env::var("ENTRYPOINT").ok().unwrap_or("test".to_string());

            let mut compiler = Compiler::new();
            let code = compiler.compile("", entrypoint)?;
            info!("{}", compiler.ir_result.unwrap().show());
            for (n, inst) in code.iter().enumerate() {
                info!("{:04} {:?}", n, inst);
            }

            Runtime::new(code.clone(), compiler.vm_code_generation.globals()).run()?;
        }
        Command::TestCompiler => {
            let mut compiler = Compiler::new();
            let code = compiler.compile("", "test".to_string())?;
            info!("{}", compiler.ir_result.unwrap().show());
            for (n, inst) in code.iter().enumerate() {
                info!("{:04} {:?}", n, inst);
            }
        }
        Command::Debug { debugger_json } => {
            let mut runtime =
                Runtime::new_from_debugger_json(debugger_json.unwrap_or("./debugger.json".into()))?;
            runtime.run().unwrap();
        }
    }

    Ok(())
}
