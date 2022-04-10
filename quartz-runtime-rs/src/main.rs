use std::io::Read;

use quartz_core::compiler::Compiler;
use runtime::Runtime;

mod runtime;

fn main() {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer).unwrap();

    let compiler = Compiler::new();
    let code = compiler.compile(&buffer).unwrap();
    println!("{:?}", Runtime::new(code).run().unwrap());
}
