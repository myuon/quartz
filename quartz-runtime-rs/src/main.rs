use std::io::Read;

use quartz_core::compiler::Compiler;

fn main() {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer).unwrap();

    let compiler = Compiler::new();
    compiler.exec(&buffer).unwrap();
}
