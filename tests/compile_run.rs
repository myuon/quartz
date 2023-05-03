use std::io::Write;

fn run_command(command: &str, args: &[&str], stdin: &[u8]) -> Option<String> {
    let mut cmd = std::process::Command::new(command);
    cmd.args(args);

    let mut child = cmd
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap().write_all(stdin).unwrap();

    let output = child.wait_with_output().unwrap();
    if !output.status.success() {
        println!("[stdout] {}", String::from_utf8(output.stdout).unwrap());
        println!("[stderr] {}", String::from_utf8(output.stderr).unwrap());
        return None;
    }

    let stdout = String::from_utf8(output.stdout).unwrap();
    Some(stdout)
}

#[test]
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
    ];

    for input in cases {
        let stdout_gen0 = run_command(
            "cargo",
            &["run", "--release", "--quiet", "--", "run", "--stdin"],
            input.as_bytes(),
        )
        .expect(format!("[INPUT]\n{}\n", input).as_str());

        let stdout = run_command(
            "cargo",
            &[
                "run",
                "--release",
                "--quiet",
                "--",
                "run",
                "./quartz/main.qz",
            ],
            input.as_bytes(),
        )
        .expect(format!("[INPUT]\n{}\n", input).as_str());
        let stdout_gen1 = run_command(
            "cargo",
            &["run", "--release", "--quiet", "--", "run-wat", "--stdin"],
            stdout.as_bytes(),
        )
        .expect(format!("[INPUT]\n{}\n[WAT]\n{}\n", input, stdout).as_str());
        assert_eq!(stdout_gen0, stdout_gen1, "[INPUT]\n{}\n", input);
    }
}
