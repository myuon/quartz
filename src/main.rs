mod runtime;
mod value;

use std::io::Read;

use anyhow::Result;
use clap::{Parser, Subcommand};
use wasmer::Value;

use crate::runtime::Runtime;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
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

#[tokio::main]
async fn main() -> Result<()> {
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
    }

    Ok(())
}
