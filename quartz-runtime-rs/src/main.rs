use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    event::{Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{error, info};
use quartz_core::{compiler::Compiler, vm::QVMSource};
use runtime::Runtime;
use std::{
    env,
    fs::File,
    io::{Read, Stdout, Write},
    path::PathBuf,
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
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
    #[clap(subcommand)]
    Debug(DebugSubCommand),
    #[clap(name = "debugger", about = "Start the Quartz debugger")]
    Debugger { debugger_json: Option<PathBuf> },
}

#[derive(Subcommand)]
enum DebugSubCommand {
    #[clap(name = "resume", about = "Resume a debug session")]
    Resume { debugger_json: Option<PathBuf> },
    #[clap(name = "step", about = "Step a debug session")]
    Step { debugger_json: Option<PathBuf> },
    #[clap(name = "show", about = "Show debug information with a debugger json")]
    Show { debugger_json: Option<PathBuf> },
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
        Command::Debug(debug) => match debug {
            DebugSubCommand::Resume { debugger_json } => {
                let mut runtime = Runtime::new_from_debugger_json(
                    debugger_json.unwrap_or("./quartz-debugger.json".into()),
                )?;
                runtime.run().unwrap();
            }
            DebugSubCommand::Step { debugger_json } => {
                let mut runtime = Runtime::new_from_debugger_json(
                    debugger_json.unwrap_or("./quartz-debugger.json".into()),
                )?;
                runtime.step().unwrap();
            }
            DebugSubCommand::Show { debugger_json } => {
                let runtime = Runtime::new_from_debugger_json(
                    debugger_json.unwrap_or("./quartz-debugger.json".into()),
                )?;

                info!("{}", runtime.debug_info());
            }
        },
        Command::Debugger { debugger_json } => {
            let mut runtime = Runtime::new_from_debugger_json(
                debugger_json.unwrap_or("./quartz-debugger.json".into()),
            )?;

            enable_raw_mode()?;

            let mut stdout = std::io::stdout();
            execute!(stdout, EnterAlternateScreen)?;
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;

            if let Err(err) = start_debugger(&mut runtime, &mut terminal) {
                error!("{:?}", err);
            }

            disable_raw_mode()?;
            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
            terminal.show_cursor()?;
        }
    }

    Ok(())
}

fn start_debugger(
    runtime: &mut Runtime,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("n:step r:resume q:quit")
                .borders(Borders::ALL);
            f.render_widget(
                Paragraph::new(runtime.debug_info()).wrap(Wrap { trim: false }),
                block.inner(size),
            );
            f.render_widget(block, size);
        })?;

        if crossterm::event::poll(Duration::from_millis(500))? {
            match crossterm::event::read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('n') => {
                        runtime.step().unwrap();
                    }
                    KeyCode::Char('r') => {
                        runtime.run().unwrap();
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    Ok(())
}
