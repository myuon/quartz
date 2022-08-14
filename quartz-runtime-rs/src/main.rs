use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    event::{Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{error, info};
use quartz_core::{compiler::Compiler, ir::IrElement, vm::QVMSource};
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
    Run {
        #[clap(long, short)]
        profile: bool,
        #[clap(long, value_parser, value_name = "FILE")]
        qirv_output: Option<PathBuf>,
    },
    #[clap(name = "test", about = "Run a Quartz test program")]
    Test,
    #[clap(name = "compile", about = "Compile a Quartz program")]
    Compile {
        #[clap(long, value_parser, value_name = "FILE")]
        qasm_output: Option<PathBuf>,
        #[clap(long, value_parser, value_name = "FILE")]
        qasmv_output: Option<PathBuf>,
        #[clap(long, value_parser, value_name = "FILE")]
        qirv_output: Option<PathBuf>,
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
    #[clap(name = "start", about = "Start a debug session")]
    Start { debugger_json: Option<PathBuf> },
    #[clap(name = "run", about = "Run the program until it hits a breakpoint")]
    Run { debugger_json: Option<PathBuf> },
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
        Command::Run {
            profile,
            qirv_output,
        } => {
            let entrypoint = env::var("ENTRYPOINT").ok().unwrap_or("main".to_string());

            let mut buffer = String::new();
            let mut stdin = std::io::stdin();
            stdin.read_to_string(&mut buffer).unwrap();

            let mut compiler = Compiler::new();
            let compiled_result = compiler.compile(&buffer, entrypoint);
            let ir = compiler
                .ir_result
                .clone()
                .unwrap_or(IrElement::nil())
                .show();

            let mut file = File::create(qirv_output.unwrap_or("./build/out.qirv".into())).unwrap();
            file.write_all(ir.as_bytes()).unwrap();

            let code = compiled_result?;

            if profile {
                let guard = pprof::ProfilerGuardBuilder::default()
                    .frequency(1000)
                    .build()
                    .unwrap();

                Runtime::new(code.clone(), compiler.vm_code_generation.globals()).run()?;

                if let Ok(report) = guard.report().build() {
                    let file = File::create("./build/prof-flamegraph.svg").unwrap();
                    report.flamegraph(file).unwrap();
                };
            } else {
                Runtime::new(code.clone(), compiler.vm_code_generation.globals()).run()?;
            }
        }
        Command::Compile {
            qasm_output,
            qasmv_output,
            qirv_output,
        } => {
            let mut buffer = String::new();
            let mut stdin = std::io::stdin();
            stdin.read_to_string(&mut buffer).unwrap();

            let mut compiler = Compiler::new();
            let code = compiler.compile_result(&buffer, "main".to_string())?;
            let ir = compiler.ir_result.clone().unwrap().show();
            info!("{}", ir);

            let mut file = File::create(qirv_output.unwrap_or("./build/out.qirv".into())).unwrap();
            file.write_all(ir.as_bytes()).unwrap();

            let mut file =
                File::create(qasmv_output.unwrap_or("./build/out.qasmv".into())).unwrap();
            file.write_all(compiler.show_qasmv(&code).as_bytes())
                .unwrap();

            let mut file = File::create(qasm_output.unwrap_or("./build/out.qasm".into())).unwrap();
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

            let mut buffer = String::new();
            let mut stdin = std::io::stdin();
            stdin.read_to_string(&mut buffer).unwrap();

            let mut compiler = Compiler::new();
            let code = compiler.compile(&buffer, entrypoint)?;

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
            DebugSubCommand::Start { debugger_json } => {
                let entrypoint = env::var("ENTRYPOINT").ok().unwrap_or("main".to_string());

                let mut buffer = String::new();
                let mut stdin = std::io::stdin();
                stdin.read_to_string(&mut buffer).unwrap();

                let mut compiler = Compiler::new();
                let code = compiler.compile(&buffer, entrypoint)?;

                let runtime = Runtime::new(code.clone(), compiler.vm_code_generation.globals());
                let mut file =
                    File::create(debugger_json.unwrap_or("./quartz-debugger.json".into())).unwrap();
                file.write_all(&serde_json::to_vec_pretty(&runtime).unwrap())
                    .unwrap();
            }
            DebugSubCommand::Run { debugger_json } => {
                let entrypoint = env::var("ENTRYPOINT").ok().unwrap_or("main".to_string());

                let mut buffer = String::new();
                let mut stdin = std::io::stdin();
                stdin.read_to_string(&mut buffer).unwrap();

                let mut compiler = Compiler::new();
                let code = compiler.compile(&buffer, entrypoint)?;

                let mut runtime = Runtime::new(code.clone(), compiler.vm_code_generation.globals());
                runtime.set_debug_mode(debugger_json.unwrap_or("./quartz-debugger.json".into()));
                runtime.run()?;
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
                .title("n:step o:step out r:resume q:quit")
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
                    KeyCode::Char('o') => {
                        runtime.step_out().unwrap();
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
