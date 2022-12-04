use anyhow::Result;
use wasmer::{imports, Instance, Memory, MemoryType, Module, Store, Value};

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {}
    }

    pub fn run(&mut self, wat: &str) -> Result<Box<[Value]>> {
        let mut store = Store::default();
        let memory = Memory::new(&mut store, MemoryType::new(1, None, false)).unwrap();
        let module = Module::new(&store, &wat)?;
        let import_object = imports! {
            "env" => {
                "memory" => memory,
            },
        };
        let instance = Instance::new(&mut store, &module, &import_object)?;

        let main = instance.exports.get_function("main")?;
        let result = main.call(&mut store, &[])?;

        Ok(result)
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
type Point = {
    x: i32,
    y: i32,
};

fun main() {
    let p = Point { x: 10, y: 20 };

    return p.y;
}
"#,
                vec![Value::I32(3)],
            ),
        ];

        for (input, expected) in cases {
            let mut compiler = Compiler::new();
            let wat = compiler.compile(input).unwrap();
            let result = runtime
                .run(&wat)
                .context(format!("\n== SOURCE\n{}\n\n== COMPILED\n{}", input, wat))
                .unwrap();
            assert_eq!(expected.as_slice(), result.as_ref());
        }
    }
}