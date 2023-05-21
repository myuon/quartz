test:
  cargo test --release

dump:
  wasm-objdump -d build/error.wasm > build/error.log

compile:
  cargo run --release -- compile --stdin

run:
  cargo run --release -- run --stdin

run_compiler:
  cargo run --release -- run ./quartz/main.qz

run_gen1:
  cargo run --release -- run-wat ./build/compiler/gen1.wat

run_wat:
  cargo run -- run-wat ./build/build.wat

test_compiler:
  cargo run --release -- run ./quartz/main.qz --entrypoint test

install:
  cargo build --release && cp target/release/quartz ~/.local/bin

fuzztest:
  cd fuzz && cargo afl build --release && cargo afl fuzz -i in -o out target/release/fuzz_target_1

build_gen1:
  sh ./build_gen1.sh
