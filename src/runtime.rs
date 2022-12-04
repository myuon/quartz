use anyhow::Result;
use wasmer::{imports, Instance, Module, Store, Value};

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {}
    }

    pub fn run(&mut self, wat: &str) -> Result<Box<[Value]>> {
        let mut store = Store::default();
        let module = Module::new(&store, &wat)?;
        let import_object = imports! {};
        let instance = Instance::new(&mut store, &module, &import_object)?;

        let main = instance.exports.get_function("main")?;
        let result = main.call(&mut store, &[])?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
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
        ];

        for (input, expected) in cases {
            let mut compiler = Compiler::new();
            let wat = compiler.compile(input).unwrap();
            let result = runtime.run(&wat).unwrap();
            assert_eq!(expected.as_slice(), result.as_ref());
        }
    }
}
