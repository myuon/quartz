#![no_main]

use libfuzzer_sys::fuzz_target;
use quartz::compiler::Compiler;
use quartz::util::path::Path;
use std::panic;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let mut compiler = Compiler::new();
        let cwd = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let result = compiler.check(&cwd, Path::empty(), &input);
        println!("{:?}", result);
    }
});
