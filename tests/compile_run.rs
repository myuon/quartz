use std::io::Write;

#[test]
fn test_compile_run() {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("run").arg("--").arg("run").arg("--stdin");

    let mut child = cmd
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(
            b"
fun main(): i32 {
    let x: i32 = 20;
    return x + 1;
}
",
        )
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "21");
}
