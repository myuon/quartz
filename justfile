test:
  cargo test

dump:
  wasm-objdump -d build/error.wasm
