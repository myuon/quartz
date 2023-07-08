use std::io::Write;

use anyhow::{anyhow, Result};
use wasmer::{imports, Function, Instance, Module, Store, Value as WasmValue};
use wasmer_wasi::WasiState;

use crate::value::Value;

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {}
    }

    pub fn _run(&mut self, wat: &str) -> Result<Box<[WasmValue]>> {
        let mut store = Store::default();
        let module = Module::new(&store, &wat)?;

        let args_string = std::env::args().collect::<Vec<_>>().join(" ");
        let args_string_len = args_string.len();

        let mut wasi_func_env = WasiState::new("quartz")
            .preopen_dir(".")?
            .finalize(&mut store)?;
        let wasi_import_object = wasi_func_env.import_object(&mut store, &module)?;

        let mut import_object = imports! {
            "env" => {
                "debug_i32" => Function::new_typed(&mut store, |i: i64| {
                    let w = Value::from_i64(i);

                    println!("[DEBUG] {:?}", w);
                    Value::i32(0).as_i64()
                }),
                "debug" => Function::new_typed(&mut store, |i: i64| {
                    let w = Value::from_i64(i);

                    println!("[DEBUG] {:?} ({:#032b} | {:#b})", w, i >> 32, i & 0xffffffff);
                    Value::i32(0).as_i64()
                }),
                "abort" => Function::new_typed(&mut store, || -> i64 {
                    panic!("[ABORT]");
                }),
                "i64_to_string_at" => Function::new_typed(&mut store, |a_value: i64, b_value: i64, at_value: i64| {
                    let a = Value::from_i64(a_value).as_i32().unwrap();
                    let b = Value::from_i64(b_value).as_i32().unwrap();
                    let at = Value::from_i64(at_value).as_i32().unwrap() as usize;

                    // FIXME: support u64
                    let bs = format!("{}", ((a as u64) << 32_u64) | (b as u64)).chars().collect::<Vec<_>>();

                    if at >= bs.len() {
                        Value::I32(-1)
                    } else {
                        Value::I32(bs[at as usize].to_digit(10).unwrap() as i32)
                    }.as_i64()
                }),
                "get_args_len" => Function::new_typed(&mut store, move || {
                    Value::I32(args_string_len as i32).as_i64()
                }),
                "get_args_at" => Function::new_typed(&mut store, move |value: i64| {
                    let v = Value::from_i64(value).as_i32().unwrap() as usize;

                    Value::Byte(args_string.chars().nth(v).unwrap() as u8).as_i64()
                }),
            }
        };
        import_object.extend(wasi_import_object.into_iter());

        let instance = Instance::new(&mut store, &module, &import_object)?;
        wasi_func_env.initialize(&mut store, &instance)?;

        let main = instance.exports.get_function("main")?;
        let result = main.call(&mut store, &[])?;

        Ok(result)
    }

    pub fn run(&mut self, input: &str) -> Result<Box<[WasmValue]>> {
        self._run(input).map_err(|err| {
            let message = err.to_string();
            // regexp test (at offset %d) against message
            let Ok(re) = regex::Regex::new(r"\(at offset (\d+)\)") else {
                return anyhow!("Original Error: {}", err);
            };
            let Some(cap) = re.captures(&message) else {
                return anyhow!("Original Error: {}", err);
            };
            let Ok(offset) = cap[1].parse::<usize>() else {
                return anyhow!("Original Error: {}", err);
            };

            let wasm = wat::parse_str(input).unwrap();

            let Ok(mut file) = std::fs::File::create("build/build.wat") else {
                return anyhow!("Original Error: {}", err);
            };
            file.write_all(input.as_bytes()).unwrap();

            let Ok(mut file) = std::fs::File::create("build/error.wasm") else {
                return anyhow!("Original Error: {}", err);
            };
            file.write_all(&wasm).unwrap();

            // offset in base-16
            let offset_hex = format!("{:x}", offset);

            anyhow!("Error at: {}\n\nOriginal Error: {}", offset_hex, err)
        })
    }
}
