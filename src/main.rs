mod ast;
mod compiler;
mod formatter;
mod generator;
mod ir;
mod ir_code_gen;
mod lexer;
mod parser;
mod runtime;
mod typecheck;
mod util;
mod value;

use std::{
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::json;
use wasmer::Value;

use crate::{
    compiler::Compiler,
    runtime::Runtime,
    util::{ident::Ident, path::Path},
};

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

        #[clap(long, short = 'o')]
        output: Option<String>,

        file: Option<String>,
    },
    #[clap(name = "run", about = "Run a file")]
    Run {
        #[clap(long)]
        stdin: bool,
        #[clap(long)]
        entrypoint: Option<String>,

        #[clap(long, short = 'o')]
        output: Option<String>,

        file: Option<String>,
    },
    #[clap(name = "run-wat", about = "Run a wat file")]
    RunWat {
        #[clap(long)]
        stdin: bool,

        file: Option<String>,
    },
    #[clap(name = "check", about = "Check a file")]
    Check {
        #[clap(long)]
        project: Option<String>,

        file: Option<String>,
    },
    #[clap(name = "check-type", about = "Get Type of a Node")]
    CheckType {
        #[clap(long)]
        project: Option<String>,
        #[clap(long)]
        line: usize,
        #[clap(long)]
        column: usize,

        file: Option<String>,
    },
    #[clap(name = "go-to-def", about = "Go To Definition")]
    GoToDef {
        #[clap(long)]
        project: Option<String>,
        #[clap(long)]
        line: usize,
        #[clap(long)]
        column: usize,

        file: Option<String>,
    },
    #[clap(name = "completion", about = "Completion")]
    Completion {
        #[clap(long)]
        project: Option<String>,
        #[clap(long)]
        line: usize,
        #[clap(long)]
        column: usize,
        #[clap(long)]
        stdin: bool,
        #[clap(long)]
        dot: bool,

        file: Option<String>,
    },
    #[clap(name = "format", about = "Format")]
    Format {
        #[clap(long)]
        write: bool,
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

#[derive(Debug)]
struct QuartzProject {
    project_path: String,
    module_path: Path,
    source: String,
}

fn load_project(file_path: &String, project_path: &Option<String>) -> Result<QuartzProject> {
    let project_path = project_path
        .clone()
        .unwrap_or(std::env::current_dir()?.to_str().unwrap().to_string());

    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut module_path = Path::new(
        file_path
            .replace(&project_path, "")
            .replace(".qz", "")
            .split("/")
            .map(|s| Ident(s.to_string()))
            .collect::<Vec<_>>(),
    );
    if module_path.0[0].0 == "" {
        module_path.0.remove(0);
    }

    Ok(QuartzProject {
        project_path,
        module_path,
        source: buffer,
    })
}

fn print_result_value(result: Box<[Value]>) {
    if result.to_vec() != vec![Value::I64(value::Value::nil().as_i64())] {
        for r in result.iter() {
            match r {
                Value::I64(t) => {
                    let v = value::Value::from_i64(*t);

                    match v {
                        value::Value::I32(p) => {
                            println!("{}", p);
                        }
                        value::Value::Byte(b) => {
                            println!("{}", b as char);
                        }
                        value::Value::Bool(b) => {
                            println!("{}", b);
                        }
                        value::Value::Pointer(0) => {
                            println!("nil");
                        }
                        value::Value::Pointer(p) => {
                            println!("<address 0x{:x}>", p);
                        }
                    }
                }
                _ => todo!(),
            }
        }
    }
}

fn main() -> Result<()> {
    let mut compiler = Compiler::new();
    let mut runtime = Runtime::new();
    if std::env::var("MODE") == Ok("run-wat".to_string()) {
        let wat_file = std::env::var("WAT_FILE")?;
        let mut file = std::fs::File::open(wat_file)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let result = runtime.run(&buffer)?;

        print_result_value(result);

        return Ok(());
    }

    let cli = Cli::parse();
    match cli.subcmd {
        SubCommand::Compile {
            stdin,
            file,
            output,
        } => {
            compile(&mut compiler, stdin, file, None, output)?;
        }
        SubCommand::Run {
            stdin,
            file,
            entrypoint,
            output,
        } => {
            let wat = compile(
                &mut compiler,
                stdin,
                file,
                entrypoint.map(|t| Ident(t)),
                output,
            )?;
            let result = runtime.run(&wat)?;

            print_result_value(result);
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

            print_result_value(result);
        }
        SubCommand::Check { project, file } => {
            let path = file.ok_or(anyhow::anyhow!("No file specified"))?;
            let mut file = std::fs::File::open(path)?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;

            let result = compiler.check(
                &project.unwrap_or(std::env::current_dir()?.to_str().unwrap().to_string()),
                Path::empty(),
                &buffer,
            );
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        SubCommand::CheckType {
            project,
            file,
            line,
            column,
        } => {
            let package = load_project(&file.unwrap(), &project)?;

            let result = compiler.check_type(
                &package.project_path,
                package.module_path,
                &package.source,
                line,
                column,
            );
            println!("{}", result);
        }
        SubCommand::GoToDef {
            project,
            file,
            line,
            column,
        } => {
            let package = load_project(&file.clone().unwrap(), &project)?;

            let path = file.ok_or(anyhow::anyhow!("No file specified"))?;
            let mut file = std::fs::File::open(path.clone())?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;

            let Ok(result) = compiler.go_to_def(
                &project.unwrap_or(std::env::current_dir()?.to_str().unwrap().to_string()),
                package.module_path,
                &buffer,
                line,
                column,
            ) else {
                return Ok(());
            };
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "file": replace_file_name(&path, &format!("{}.qz", result.module_name)),
                    "start": {
                        "line": result.start.0,
                        "column": result.start.1,
                    },
                    "end": {
                        "line": result.end.0,
                        "column": result.end.1,
                    }
                }))?
            );
        }
        SubCommand::Completion {
            project,
            line,
            column,
            file,
            stdin,
            dot,
        } => {
            let package = load_project(&file.unwrap(), &project)?;
            let input = if stdin {
                read_from_stdin()
            } else {
                package.source
            };

            let result = compiler.completion(
                &package.project_path,
                package.module_path,
                &input,
                line,
                column,
                dot,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &json!({ "items": result.into_iter().map(|(kind, label, type_)| 
                        json!({
                            "kind": kind,
                            "label": label,
                            "detail": type_,
                        })
                    ).collect::<Vec<_>>() })
                )?
            );
        }
        SubCommand::Format { write, file, stdin } => {
            if stdin {
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer)?;

                let formatted = Compiler::format(&buffer)?;
                println!("{}", formatted);
            } else {
                let path = file.ok_or(anyhow::anyhow!("No file specified"))?;
                let mut file = std::fs::File::open(path.clone())?;
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;

                let formatted = Compiler::format(&buffer)?;

                if write {
                    let mut file = std::fs::File::create(path)?;
                    file.write_all(formatted.as_bytes())?;
                } else {
                    println!("{}", formatted);
                }
            }
        }
    }

    Ok(())
}

fn replace_file_name(url: &str, new_file_name: &str) -> String {
    let path_str = url;
    let mut path = PathBuf::from(path_str);

    path.set_file_name(new_file_name);

    path.to_string_lossy().to_string()
}

fn compile(
    compiler: &mut Compiler,
    stdin: bool,
    file: Option<String>,
    entrypoint_name: Option<Ident>,
    output: Option<String>,
) -> Result<String> {
    let cwd = std::env::current_dir()?.to_str().unwrap().to_string();
    let input = if stdin {
        read_from_stdin()
    } else {
        let package = load_project(&file.unwrap(), &None)?;

        package.source
    };
    let wat = compiler.compile(&cwd, &input, entrypoint_name)?;

    let file = std::fs::File::create(output.unwrap_or("build/build.wat".to_string()))?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(wat.as_bytes())?;

    let file = std::fs::File::create("build/build.wasm")?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(&wat::parse_str(&wat)?)?;

    Ok(wat)
}
