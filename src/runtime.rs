use std::io::Write;

use anyhow::{anyhow, Result};
use wasmer::{imports, Function, Instance, MemoryView, Module, Store, Value as WasmValue};
use wasmer_wasi::WasiState;

use crate::value::Value;

#[derive(Debug, Clone, Copy)]
struct ExitCode(u32);

impl std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ExitCode {}

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {}
    }

    fn dump_memory(view: MemoryView) -> Result<()> {
        if let Ok(file_path) = std::env::var("MEMORY_DUMP_FILE") {
            let mut file = std::fs::File::create(file_path.clone()).unwrap();

            let mut offset = 0;
            let mut chunk = [0u8; 40960];
            while offset < view.data_size() {
                let remaining = view.data_size() - offset;
                let sublen = remaining.min(chunk.len() as u64) as usize;
                view.read(offset, &mut chunk[..sublen])?;

                file.write_all(&chunk[..sublen]).unwrap();

                offset += sublen as u64;
            }

            println!("Memory dumped to file: {}", file_path);
        }

        Ok(())
    }

    pub fn _run(&mut self, wat: &str) -> Result<Box<[WasmValue]>> {
        let mut store = Store::default();
        let module = Module::new(&store, &wat)?;

        let args = std::env::args().collect::<Vec<_>>();
        let envs = std::env::vars()
            .map(|(k, v)| (k, v))
            .collect::<Vec<_>>()
            .into_iter();

        let mut wasi_func_env = WasiState::new("quartz")
            .preopen_dir(".")?
            .envs(envs)
            .args(args[1..].to_vec())
            .finalize(&mut store)?;
        let wasi_import_object = wasi_func_env.import_object(&mut store, &module)?;

        fn abort() -> Result<i64, ExitCode> {
            println!("[ABORT]");

            Err(ExitCode(1))
        }

        let mut import_object = imports! {
            "env" => {
                "debug" => Function::new_typed(&mut store, |i: i64| {
                    let w = Value::from_i64(i);

                    println!("[DEBUG] {} ({:#032b} | {:#b})", match w {
                        Value::Pointer(p) => format!("Pointer(0x{:x})", p),
                        _ => format!("{:?}", w),
                    }, (i >> 32) as i32, i & 0xffffffff);
                    Value::i32(0).as_i64()
                }),
                "abort" => Function::new_typed(&mut store, abort),
                // @Deprecated: will be removed in 2.3.0+
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
            }
        };
        import_object.extend(wasi_import_object.into_iter());

        let instance = Instance::new(&mut store, &module, &import_object)?;
        wasi_func_env.initialize(&mut store, &instance)?;

        let main = instance.exports.get_function("main")?;
        let result = main
            .call(&mut store, &[])
            .map_err(|err| anyhow!("calling main: {:?}", err));

        let memory = instance.exports.get_memory("memory")?;
        let view = memory.view(&mut store);

        Runtime::dump_memory(view)?;

        result
    }

    pub fn run(&mut self, input: &str) -> Result<Box<[WasmValue]>> {
        self._run(input).map_err(|err| {
            let message = err.to_string();
            // regexp test (at offset %d) against message
            let Ok(re) = regex::Regex::new(r"\(at offset (\d+)\)") else {
                return anyhow!("Original Error: {:?}", err);
            };
            let Some(cap) = re.captures(&message) else {
                return anyhow!("Original Error: {:?}", err);
            };
            let Ok(offset) = cap[1].parse::<usize>() else {
                return anyhow!("Original Error: {:?}", err);
            };

            let wasm = wat::parse_str(input).unwrap();

            let Ok(mut file) = std::fs::File::create("build/build.wat") else {
                return anyhow!("Original Error: {:?}", err);
            };
            file.write_all(input.as_bytes()).unwrap();

            let Ok(mut file) = std::fs::File::create("build/error.wasm") else {
                return anyhow!("Original Error: {:?}", err);
            };
            file.write_all(&wasm).unwrap();

            // offset in base-16
            let offset_hex = format!("{:x}", offset);

            anyhow!("Error at: {}\n\nOriginal Error: {:?}", offset_hex, err)
        })
    }
}
