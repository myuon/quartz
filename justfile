test:
  cargo test --release

dump:
  wasm-objdump -d build/error.wasm > build/error.log

compile:
  cargo run --release -- compile --stdin

run:
  cargo run --release -- run --stdin

run_gen0_compiler:
  cargo run --release -- run ./quartz/main.qz

build_gen1:
  cargo run --release -- compile ./build/compiler/compiler.qz && mv ./build/build.wat ./build/compiler/gen1.wat

run_gen1:
  cargo run --release -- run-wat ./build/compiler/gen1.wat

build_gen2:
  cargo run --release -- run-wat ./build/compiler/gen1.wat < ./build/compiler/compiler.qz > ./build/compiler/gen2.wat

build_gen3:
  cargo run --release -- run-wat ./build/compiler/gen2.wat < ./build/compiler/compiler.qz > ./build/compiler/gen3.wat

run_wat:
  cargo run -- run-wat ./build/build.wat

test_compiler:
  cargo run --release -- run ./quartz/main.qz --entrypoint test

install:
  cargo build --release && cp target/release/quartz ~/.local/bin

fuzztest:
  cd fuzz && cargo afl build --release && cargo afl fuzz -i in -o out target/release/fuzz_target_1

build_compiler_source:
  sh ./build_compiler_source.sh

test_self_compile:
  just build_compiler_source && just build_gen1 && just build_gen2 && just build_gen3 && diff ./build/compiler/gen2.wat ./build/compiler/gen3.wat
