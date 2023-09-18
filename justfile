install:
  ln -s $(pwd)/quartzcli ~/.local/bin/quartz
  ln -s $(pwd)/target/release/quartz ~/.local/bin/quartz_bin
  ln -s $(pwd)/build/quartz-current.wat ~/.local/bin/quartz-current.wat

download version:
	@echo "Downloading version {{version}}"
	@wget -P ./build https://github.com/myuon/quartz/releases/download/v{{version}}/quartz-{{version}}.wat

build_compiler version new_version options="":
  @echo "Building compiler {{version}} -> {{new_version}}"
  @MODE=run-wat WAT_FILE=./build/quartz-{{version}}.wat cargo run --release -- compile {{options}} -o ./build/quartz-{{new_version}}.wat ./quartz/main.qz

upload new_version:
  @echo "Uploading version {{new_version}}"
  @gh release upload v{{new_version}} ./build/quartz-{{new_version}}.wat
  @wat2wasm ./build/quartz-{{new_version}}.wat -o ./build/quartz-{{new_version}}.wasm
  @gh release upload v{{new_version}} ./build/quartz-{{new_version}}.wasm
  @gsutil cp build/quartz-{{new_version}}.wasm gs://quartz-playground.appspot.com/quartz

find_latest_version:
  @curl -s https://api.github.com/repos/myuon/quartz/releases/latest | jq -r '.tag_name' | tr -d 'v'

find_latest_version_local:
  @git tag | grep -v 'rc' | sort -V | tail -n 1 | tr -d 'v'

download_latest:
  @just download $(just find_latest_version)

build_current_compiler:
  @just build_compiler $(just find_latest_version_local) current

run file options="":
  @just build_current_compiler
  MODE=run-wat WAT_FILE=./build/quartz-current.wat cargo run --release -- compile {{options}} -o ./build/quartz-compiled.wat {{file}}
  MEMORY_DUMP_FILE=./build/memory/memory.dat MODE=run-wat WAT_FILE=./build/quartz-compiled.wat cargo run --release

test file options="":
  @just build_current_compiler
  MODE=run-wat WAT_FILE=./build/quartz-current.wat cargo run --release -- compile {{options}} --test -o ./build/quartz-compiled.wat {{file}}
  MODE=run-wat WAT_FILE=./build/quartz-compiled.wat cargo run --release

check_if_stable options="":
  @just build_compiler current current.2 {{options}} && just build_compiler current.2 current.3 {{options}} && just build_compiler current.3 current.4 {{options}} && diff -w build/quartz-current.3.wat build/quartz-current.4.wat

test_quartz:
  just test ./quartz/main.qz --validate-address
