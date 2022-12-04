mod ast;
mod compiler;
mod generator;
mod lexer;
mod parser;

use wasmer::{imports, Instance, Module, Store, Value};

use crate::compiler::Compiler;

fn main() -> anyhow::Result<()> {
    let mut compiler = Compiler::new();
    let wat = compiler.compile(
        r#"
fun main(): i32 {
  let x = 10;
  return x + 1;
}
"#,
    )?;

    let mut store = Store::default();
    let module = Module::new(&store, &wat)?;
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;
    let result = add_one.call(&mut store, &[Value::I32(42)])?;
    assert_eq!(result[0], Value::I32(43));

    println!("{:?}", result);

    Ok(())
}
