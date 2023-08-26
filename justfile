install:
  ln -s $(pwd)/quartzcli ~/.local/bin/quartz

download version:
	@echo "Downloading version {{version}}"
	@wget -P ./build https://github.com/myuon/quartz/releases/download/v{{version}}/quartz-{{version}}.wat

build_compiler version new_version options="":
  @echo "Building compiler {{version}} -> {{new_version}}"
  @MODE=run-wat WAT_FILE=./build/quartz-{{version}}.wat cargo run --release -- compile {{options}} -o ./build/quartz-{{new_version}}.wat ./quartz/main.qz
  wat2wasm ./build/quartz-{{new_version}}.wat -o ./build/quartz-{{new_version}}.wasm

upload new_version:
  @echo "Uploading version {{new_version}}"
  @gh release upload v{{new_version}} ./build/quartz-{{new_version}}.wat

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
  MODE=run-wat WAT_FILE=./build/quartz-compiled.wat cargo run --release

run_2 file options="":
  @just build_current_compiler
  @just build_compiler current current.2 {{options}}
  MODE=run-wat WAT_FILE=./build/quartz-current.2.wat cargo run --release -- compile {{options}} -o ./build/quartz-compiled.wat {{file}}
  MODE=run-wat WAT_FILE=./build/quartz-compiled.wat cargo run --release

check_if_stable options="":
  @just build_compiler current current.2 {{options}} && just build_compiler current.2 current.3 {{options}} && just build_compiler current.3 current.4 {{options}} && diff -w build/quartz-current.3.wat build/quartz-current.4.wat
