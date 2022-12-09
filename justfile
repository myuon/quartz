test:
  cargo test

dump:
  wasm-objdump -d build/error.wasm

compile:
  cargo run -- compile
