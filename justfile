test:
  cargo test --release

dump:
  wasm-objdump -d build/error.wasm > build/error.log

compile:
  cargo run -- compile --stdin

run:
  cargo run --release -- run --stdin

run_compiler:
  cargo run --release -- run ./quartz/main.qz

run_wat:
  cargo run -- run-wat ./build/build.wat
