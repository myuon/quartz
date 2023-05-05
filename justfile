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

test_compiler:
  cargo run --release -- run ./quartz/main.qz --entrypoint test

install:
  cargo build --release && cp target/release/quartz ~/.local/bin

fuzztest:
  cargo +nightly fuzz run fuzz_target_1
