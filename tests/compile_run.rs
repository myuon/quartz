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
        return output_compile.stdout;
    }

    let output_run = quartz_run_wat(output_path);
    output_run.stdout
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
                let expected = fs::read_to_string(stdout_path)?;
                assert_eq!(output, expected);
            } else {
                println!("[stdout] {}", output);
            }
        }
    }

    Ok(())
}

// #[test]
fn test_compile_run() {
    let cases = vec![
        r#"
fun main(): i32 {
    return 1;
}
"#,
        r#"
fun main(): i32 {
    let x: i32 = 20;
    return x + 1;
}
"#,
        r#"
fun calc(b: i32): i32 {
    let a = 1;
    let z = 10;
    return z + a + b;
}

fun main(): i32 {
    return calc(2);
}
"#,
        r#"
let a = 5;

fun f() {
    a = a + 10;
}

fun main(): i32 {
    f();
    return a;
}
"#,
        r#"
fun factorial(n: i32): i32 {
    if n == 0 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }

    return 0;
}

fun main(): i32 {
    return factorial(5);
}
"#,
        r#"
fun main(): i32 {
    let x = 10;
    let n = 0;
    while n < 10 {
        x = x + n;
        n = n + 1;
    }

    return x;
}
        "#,
        r#"
struct Position {
    x: i32,
    y: i32,
}

fun main(): i32 {
    let p = Position {
        x: 10,
        y: 20,
    };

    return p.y;
}
        "#,
        r#"
struct Point {
    x: i32,
    y: i32,
}

fun main(): i32 {
    let p = Point { x: 10, y: 20 };

    return p.x;
}
"#,
        r#"
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fun main(): i32 {
    let p = Point { x: 10, y: 20, z: 0 };
    p.z = p.x + p.y;

    return p.z;
}
"#,
        r#"
fun main(): i32 {
    let p = make[ptr[i32]](20);
    p.at(0) = 10;
    p.at(1) = 20;
    p.at(2) = p.at(0) + p.at(1);

    return p.at(2);
}
"#,
        r#"
struct Point {
    x: i32,
    y: i32,
}

module Point {
    fun get_x(self): i32 {
        return self.x;
    }

    fun sum(self): i32 {
        return self.get_x() + self.y;
    }

    fun new(x: i32, y: i32): Point {
        return Point {
            x: x,
            y: y,
        };
    }
}

fun main(): i32 {
    let p = Point::new(10, 20);

    return p.sum();
}
"#,
        r#"
fun new_string(p: ptr[byte], length: i32): string {
    return string {
        data: p,
        length: length,
    };
}

fun main(): byte {
    let p = make[ptr[byte]](3);
    p.at(0) = 48 as byte;
    p.at(1) = 56 as byte;
    p.at(2) = 72 as byte;

    let s = new_string(p, 3);
    return s.at(0);
}
"#,
        r#"
fun main(): byte {
    let a = "hello";

    return a.at(3);
}
"#,
        r#"
fun int_to_string(n: i32): string {
    let digit = 0;
    let tmp = n;
    while tmp > 0 {
        tmp = tmp / 10;
        digit = digit + 1;
    }

    let str = make[ptr[byte]](digit);
    tmp = n;
    for i in 0..digit {
        let d = tmp % 10;
        str.at(digit - i - 1) = ((d + 48) as byte);
        tmp = tmp / 10;
    }

    return string {
        data: str,
        length: digit,
    };
}

fun main(): i32 {
    let str = int_to_string(123456);

    return str.data.at(5) as i32 - 48;
}
"#,
        r#"
fun f(): string {
    let s = "foo"?;

    if true {
        return s!;
    } else {
        return "";
    }
}

fun main(): i32 {
    return 0;
}
"#,
        r#"
fun main(): i32 {
    if "hello".equal("hello") {
        return 0;
    } else {
        return 1;
    }
}
"#,
        r#"
fun hoge(): string? {
    return "hoge"?;
}

fun main(): bool {
    let t = hoge();

    return t != nil;
}
"#,
        r#"
fun vec_len(..t: vec[i32]): i32 {
    return t.length;
}

fun main(): i32 {
    let t1 = vec_len(1,2,3,4);
    let t2 = vec_len(1,2,3,4,5);

    return t1 + t2;
}
"#,
        r#"
fun f(a: i32): i32 or string {
    if a == 0 {
        return _ or "zero";
    } else {
        return a;
    }
}

fun main(): bool {
    let a or b = f(0);
    return a == nil && b!.equal("zero");
}
"#,
        r#"
fun main(): i32 {
    let p = make[vec[i32]]();
    for i in 0..100 {
        p.push(i);
    }

    return p.at(40) + p.at(60);
}
"#,
        r#"
enum Value {
    t_i32: i32,
    t_string: string,
}

fun main(): i32 {
    let t_10 = Value { t_i32: 10 };
    let t_hello = Value { t_string: "hello" };

    return t_10.t_i32! + t_hello.t_string!.length;
}
"#,
        r#"
enum Value {
    t_i32: i32,
    t_string: string,
}

module Value {
    fun get(self): i32 {
        if self.t_i32 != nil {
            return self.t_i32!;
        }
        if self.t_string != nil {
            return self.t_string!.length;
        }

        return 0;
    }
}

fun main(): i32 {
    let t_10 = Value { t_i32: 10 };
    let t_hello = Value { t_string: "hello" };

    return t_10.get() + t_hello.get();
}
"#,
        r#"
struct Foo {
    x: bool,
}

module Foo {
    fun run(self, ..args: vec[i32]): i32 {
        return args.length;
    }
}

fun main(): i32 {
    let foo = Foo {
        x: true,
    };
    let t = make[vec[i32]](1,2,3,4,5);

    return foo.run(..t);
}
"#,
    ];

    // for input in cases {
    //     let stdout_gen0 = run_command(
    //         "cargo",
    //         &["run", "--release", "--quiet", "--", "run", "--stdin"],
    //         input.as_bytes(),
    //         vec![],
    //     )
    //     .expect(format!("[INPUT:gen0]\n{}\n", input).as_str());

    //     let stdout = run_command(
    //         "cargo",
    //         &["run", "--release", "--quiet", "--", "run"],
    //         input.as_bytes(),
    //         vec![("WAT_FILE", "./build/gen1.wat"), ("MODE", "run-wat")]
    //             .into_iter()
    //             .map(|(k, v)| (k.to_string(), v.to_string()))
    //             .collect(),
    //     )
    //     .expect(format!("[INPUT:gen1:compile]\n{}\n", input).as_str());
    //     let stdout_gen1 = run_command(
    //         "cargo",
    //         &["run", "--release", "--quiet", "--", "run-wat", "--stdin"],
    //         stdout.as_bytes(),
    //         vec![],
    //     )
    //     .expect(
    //         format!(
    //             "[INPUT:gen1:runtime]\n{}\n[WAT]\n{}\n",
    //             input,
    //             &stdout[0..100]
    //         )
    //         .as_str(),
    //     );
    //     assert_eq!(stdout_gen0, stdout_gen1, "[INPUT]\n{}\n", input);
    // }
}
