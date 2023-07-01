install:
  ln -s $(pwd)/quartzcli ~/.local/bin/quartz

download version:
	@echo "Downloading version {{version}}"
	@wget -P ./build https://github.com/myuon/quartz/releases/download/v{{version}}/quartz-{{version}}.wat

build_compiler version new_version:
  @echo "Building compiler {{version}} -> {{new_version}}"
  @MODE=run-wat WAT_FILE=./build/quartz-{{version}}.wat cargo run --release -- compile -o ./build/quartz-{{new_version}}.wat ./quartz/main.qz

upload new_version:
  @echo "Uploading version {{new_version}}"
  @gh release upload v{{new_version}} ./build/quartz-{{new_version}}.wat

find_latest_version:
  @git tag | grep -v 'rc' | sort -V | tail -n 1 | tr -d 'v'

download_latest:
  @just download $(just find_latest_version)

build_current_compiler:
  @just build_compiler $(just find_latest_version) current

run file:
  @just build_current_compiler
  MODE=run-wat WAT_FILE=./build/quartz-current.wat cargo run --release -- compile -o ./build/quartz-compiled.wat {{file}}
  MODE=run-wat WAT_FILE=./build/quartz-compiled.wat cargo run --release
