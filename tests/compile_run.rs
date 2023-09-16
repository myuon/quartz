use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

struct RunCommandOutput {
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
        .spawn()
        .unwrap();
    child.stdin.as_mut()?.write_all(stdin).ok()?;

    let output = child.wait_with_output().unwrap();

    Some(RunCommandOutput {
        success: output.status.success(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    })
}

fn quartz_compile(input_path: &Path, output_path: &Path) -> RunCommandOutput {
    run_command(
        &format!("./target/release/quartz",),
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

fn quartz_run(input_path: &Path, output_path: &Path) -> String {
    let output_compile = quartz_compile(input_path, output_path);
    if !output_compile.success {
        return format!("{}\n{}", output_compile.stdout, output_compile.stderr);
    }

    let output_run = quartz_run_wat(output_path);
    format!("{}\n{}", output_run.stdout, output_run.stderr)
}

#[test]
fn test_run() -> Result<()> {
    for entry in WalkDir::new("./tests/cases")
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str());
        if ext == Some("qz") {
            let stdout_path = path.with_extension("stdout");
            let compiled_path = Path::new("/tmp/compiled.wat");
            let output = quartz_run(path, compiled_path);

            if stdout_path.exists() {
                let expected_fragment = fs::read_to_string(stdout_path)?;
                assert!(
                    output.contains(&expected_fragment),
                    "{}\n{}",
                    path.display(),
                    output
                );
            } else {
                println!("[stdout] {}", output);
            }
        }
    }

    Ok(())
}
