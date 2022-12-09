use std::io::Write;

use anyhow::{anyhow, Result};
use wasmer::{imports, Instance, Module, Store};
use wasmer::{Function, Value};

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {}
    }

    pub fn _run(&mut self, wat: &str) -> Result<Box<[Value]>> {
        let mut store = Store::default();
        let module = Module::new(&store, &wat)?;
        let import_object = imports! {
            "env" => {
                "write_stdout" => Function::new_typed(&mut store, |ch: u32| {
                    std::io::stdout().lock().write_all(&ch.to_be_bytes()).unwrap();
                }),
            }
        };
        let instance = Instance::new(&mut store, &module, &import_object)?;

        let main = instance.exports.get_function("main")?;
        let result = main.call(&mut store, &[])?;

        Ok(result)
    }

    pub fn run(&mut self, input: &str) -> Result<Box<[Value]>> {
        self._run(input).map_err(|err| {
            let message = err.to_string();
            // regexp test (at offset %d) against message
            let Ok(re) = regex::Regex::new(r"\(at offset (\d+)\)") else {
                return anyhow!("{}\n\nOriginal Error: {}", input, err);
            };
            let Some(cap) = re.captures(&message) else {
                return anyhow!("{}\n\nOriginal Error: {}", input, err);
            };
            let Ok(offset) = cap[1].parse::<usize>() else {
                return anyhow!("{}\n\nOriginal Error: {}", input, err);
            };

            let wasm = wat::parse_str(input).unwrap();

            let Ok(mut file) = std::fs::File::create("build/build.wat") else {
                return anyhow!("{}\n\nOriginal Error: {}", input, err);
            };
            file.write_all(input.as_bytes()).unwrap();

            let Ok(mut file) = std::fs::File::create("build/error.wasm") else {
                return anyhow!("{}\n\nOriginal Error: {}", input, err);
            };
            file.write_all(&wasm).unwrap();

            // offset in base-16
            let offset_hex = format!("{:x}", offset);

            anyhow!("Error at: {}\n\nOriginal Error: {}", offset_hex, err)
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use pretty_assertions::assert_eq;

    use crate::compiler::Compiler;

    use super::*;

    #[test]
    fn test_compile_and_run() {
        let mut runtime = Runtime::new();
        let cases = vec![
            (
                r#"
fun main(): i32 {
    let x: i32 = 10;
    return x + 1;
}
"#,
                vec![Value::I32(11)],
            ),
            (
                r#"
fun main() {
    let x = 10;
    return x + 1;
}
"#,
                vec![Value::I32(11)],
            ),
            (
                r#"
fun calc(b: i32): i32 {
    let a = 1;
    let z = 10;
    return z + a + b;
}

fun main() {
    return calc(2);
}
"#,
                vec![Value::I32(13)],
            ),
            (
                r#"
let a = 5;

fun f() {
    a = a + 10;
}

fun main() {
    f();
    return a;
}
"#,
                vec![Value::I32(15)],
            ),
            (
                r#"
fun factorial(n: i32): i32 {
    if n == 0 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

fun main() {
    return factorial(5);
}
"#,
                vec![Value::I32(120)],
            ),
            (
                r#"
fun main() {
    let x = 10;
    let n = 0;
    while n < 10 {
        x = x + n;
        n = n + 1;
    }

    return x;
}
"#,
                vec![Value::I32(55)],
            ),
            (
                r#"
type Point = {
    x: i32,
    y: i32,
};

fun main() {
    let p = Point { x: 10, y: 20 };

    return p.y;
}
"#,
                vec![Value::I32(20)],
            ),
            (
                r#"
type Point = {
    x: i32,
    y: i32,
    z: i32,
};

fun main() {
    let p = Point { x: 10, y: 20, z: 0 };
    p.z = p.x + p.y;

    return p.z;
}
"#,
                vec![Value::I32(30)],
            ),
            (
                r#"
fun main() {
    let p = make[array[i32,20]]();
    p.at(0) = 10;
    p.at(1) = 20;
    p.at(2) = p.at(0) + p.at(1);

    return p.at(2);
}
"#,
                vec![Value::I32(30)],
            ),
            (
                r#"
fun main() {
    let str = "Hello, World!";

    return println(str);
}
"#,
                vec![],
            ),
            (
                r#"
fun h(b: i32): i32 {
    return b;
}

fun g(b: i32): i32 {
    let a = 20;

    return h(b) + a;
}

fun f(b: i32): i32 {
    let a = 10;

    return g(b) + a;
}

fun main() {
    let a = 5;
    return f(a);
}
"#,
                vec![Value::I32(35)],
            ),
            (
                r#"
type Point = {
    x: i32,
    y: i32,
};

fun get_x(p: Point): i32 {
    return p.x;
}

fun main() {
    return get_x(Point {
        x: 10,
        y: 20,
    });
}
"#,
                vec![Value::I32(35)],
            ),
        ];

        for (input, expected) in cases {
            let mut compiler = Compiler::new();
            let wat = compiler.compile(input).unwrap();
            let result = runtime
                .run(&wat)
                .context(format!("\n== SOURCE\n{}\n\n== COMPILED\n{}", input, wat))
                .unwrap();
            assert_eq!(expected.as_slice(), result.as_ref(), "case: {}", input);
        }
    }
}
