test:
  cargo test --release

dump:
  wasm-objdump -d build/error.wasm > build/error.log

compile:
  cargo run --release -- compile --stdin

run:
  cargo run --release -- run --stdin

run_gen0:
  cargo run --release -- run ./quartz/main.qz

build_gen1:
  cargo run --release -- compile -o ./build/gen1.wat ./quartz/main.qz

run_gen1:
  cargo run --release -- run-wat ./build/gen1.wat

build_gen2:
  MODE=run-wat WAT_FILE=./build/gen1.wat cargo run --release -- compile -o ./build/gen2.wat ./quartz/main.qz

build_gen3:
  MODE=run-wat WAT_FILE=./build/gen2.wat cargo run --release -- compile -o ./build/gen3.wat ./quartz/main.qz

run_wat:
  cargo run -- run-wat ./build/build.wat

test_compiler:
  cargo run --release -- run ./quartz/main.qz --entrypoint test

install:
  cargo build --release && cp target/release/quartz ~/.local/bin

fuzztest:
  cd fuzz && cargo afl build --release && cargo afl fuzz -i in -o out target/release/fuzz_target_1

test_self_compile:
  just build_gen1 && just build_gen2 && just build_gen3 && diff ./build/gen2.wat ./build/gen3.wat
