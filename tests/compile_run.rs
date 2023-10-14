use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
struct RunCommandOutput {
    status: std::process::ExitStatus,
    success: bool,
    stdout: String,
    stderr: String,
}

fn run_command(
    command: &str,
    args: &[&str],
    stdin: &[u8],
    envs: Vec<(String, String)>,
) -> Option<RunCommandOutput> {
    let mut cmd = std::process::Command::new(command);
    cmd.args(args);
    cmd.envs(envs);

    let mut child = cmd
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.as_mut()?.write_all(stdin).ok()?;

    let output = child
        .wait_with_output()
        .context("wait_with_output")
        .unwrap();

    Some(RunCommandOutput {
        status: output.status,
        success: output.status.success(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    })
}

fn quartz_compile(input_path: &Path, output_path: &Path) -> RunCommandOutput {
    run_command(
        "./target/release/quartz",
        &[
            "compile",
            "--validate-address",
            "-o",
            output_path.to_str().unwrap(),
            input_path.to_str().unwrap(),
        ],
        &[],
        vec![
            ("WAT_FILE", "./build/quartz-current.wat"),
            ("MODE", "run-wat"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect(),
    )
    .unwrap()
}

fn quartz_run_wat(input_path: &Path) -> RunCommandOutput {
    run_command(
        "./target/release/quartz",
        &[],
        &[],
        vec![
            ("WAT_FILE", input_path.to_str().unwrap()),
            ("MODE", "run-wat"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect(),
    )
    .unwrap()
}

fn quartz_check(input_path: &Path) -> RunCommandOutput {
    run_command(
        "./target/release/quartz",
        &["check", input_path.to_str().unwrap()],
        &[],
        vec![
            ("WAT_FILE", "./build/quartz-current.wat"),
            ("MODE", "run-wat"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect(),
    )
    .unwrap()
}

fn quartz_completion(input_path: &Path, arg: &str) -> RunCommandOutput {
    run_command(
        "./target/release/quartz",
        &["completion", arg, input_path.to_str().unwrap()],
        &[],
        vec![
            ("WAT_FILE", "./build/quartz-current.wat"),
            ("MODE", "run-wat"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect(),
    )
    .unwrap()
}

#[derive(Clone, Debug)]
struct QuartzRunOutput {
    compile: RunCommandOutput,
    run: Option<RunCommandOutput>,
}

fn quartz_run(input_path: &Path, output_path: &Path) -> QuartzRunOutput {
    let output_compile = quartz_compile(input_path, output_path);
    if !output_compile.success {
        return QuartzRunOutput {
            compile: output_compile,
            run: None,
        };
    }

    let output_run = quartz_run_wat(output_path);
    QuartzRunOutput {
        compile: output_compile,
        run: Some(output_run),
    }
}

#[test]
fn test_run() -> Result<()> {
    use rayon::prelude::*;

    WalkDir::new("./tests/cases")
        .into_iter()
        .filter_map(Result::ok)
        .enumerate()
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(i, entry)| -> Result<()> {
            let path = entry.path();
            let ext = path.extension().and_then(|s| s.to_str());
            if ext == Some("qz") {
                let stdout_path = path.with_extension("stdout");
                let stderr_path = path.with_extension("stderr");
                let name = format!("/tmp/compiled_{}.wat", i);
                let compiled_path = Path::new(&name);
                let output = quartz_run(path, compiled_path);

                if stdout_path.exists() {
                    let expected = fs::read_to_string(stdout_path)?;
                    let run = output.run.clone().expect(
                        format!(
                            "output.run is None.\n{}\n[COMPILE]: {}\n{}",
                            path.display(),
                            output.compile.stdout,
                            output.compile.stderr
                        )
                        .as_str(),
                    );

                    assert_eq!(
                        expected,
                        run.stdout,
                        "{}\n\n[compile]\n{}\n[run]\n{}",
                        path.display(),
                        output.compile.stdout,
                        run.stdout,
                    );
                }
                if stderr_path.exists() {
                    let expected_fragment = fs::read_to_string(stderr_path)?;
                    let run = output.run.unwrap();
                    let actual = format!("{}\n{}", output.compile.stderr, run.stderr);

                    assert!(
                        actual.contains(&expected_fragment),
                        "{}\n\n[expected]\n{}\n[actual]\n{}\n",
                        path.display(),
                        expected_fragment,
                        actual,
                    );
                }

                let _ = std::fs::remove_file(compiled_path);
            }

            Ok(())
        })
        .collect::<Result<_>>()?;

    Ok(())
}

#[test]
fn test_check() -> Result<()> {
    use rayon::prelude::*;

    WalkDir::new("./tests/lsp_check")
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|entry| -> Result<()> {
            let path = entry.path();
            let ext = path.extension().and_then(|s| s.to_str());
            if ext == Some("qz") {
                let stdout_path = path.with_extension("stdout");
                let stderr_path = path.with_extension("stderr");
                let output = quartz_check(path);

                assert!(output.status.success(), "{}", path.display());

                if stdout_path.exists() {
                    let expected = fs::read_to_string(stdout_path)?;
                    assert_eq!(expected, output.stdout, "{}", path.display(),);
                }
                if stderr_path.exists() {
                    let expected_fragment = fs::read_to_string(stderr_path)?;

                    assert!(
                        output.stderr.contains(&expected_fragment),
                        "{}\n\n[expected]\n{}\n[actual]\n{}\n",
                        path.display(),
                        expected_fragment,
                        output.stderr,
                    );
                }
            }

            Ok(())
        })
        .collect::<Result<_>>()?;

    Ok(())
}

#[test]
fn test_completion() -> Result<()> {
    use rayon::prelude::*;

    WalkDir::new("./tests/lsp_completion")
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|entry| -> Result<()> {
            let path = entry.path();
            let ext = path.extension().and_then(|s| s.to_str());
            if ext == Some("qz") {
                let stdout_path = path.with_extension("stdout");
                let stderr_path = path.with_extension("stderr");
                let arg_path = path.with_extension("arg");
                let output = quartz_completion(path, &fs::read_to_string(arg_path)?);

                assert!(
                    output.status.success(),
                    "{}\n[stderr] {}\n[stdout] {}",
                    path.display(),
                    output.stderr,
                    output.stdout,
                );

                if stdout_path.exists() {
                    let expected = fs::read_to_string(stdout_path)?;
                    assert_eq!(expected, output.stdout, "{}", path.display(),);
                }
                if stderr_path.exists() {
                    let expected_fragment = fs::read_to_string(stderr_path)?;

                    assert!(
                        output.stderr.contains(&expected_fragment),
                        "{}\n\n[expected]\n{}\n[actual]\n{}\n",
                        path.display(),
                        expected_fragment,
                        output.stderr,
                    );
                }
            }

            Ok(())
        })
        .collect::<Result<_>>()?;

    Ok(())
}
