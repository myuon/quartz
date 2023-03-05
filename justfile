test:
  cargo test

dump:
  wasm-objdump -d build/error.wasm > build/error.log

compile:
  cargo run -- compile

run:
  cargo run -- run

run_compiler:
  cargo run -- run ./quartz/main.qz
