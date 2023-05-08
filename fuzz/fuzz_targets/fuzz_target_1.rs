#[macro_use]
extern crate afl;

use quartz::compiler::Compiler;
use quartz::util::path::Path;
use std::panic;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(input) = std::str::from_utf8(data) {
            let mut compiler = Compiler::new();
            compiler.check(".", Path::empty(), &input);
        }
    });
}
