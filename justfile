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
  @curl -s https://api.github.com/repos/myuon/quartz/releases/latest | jq -r '.tag_name' | tr -d 'v'

download_latest:
  @just download $(just find_latest_version)

build_current_compiler:
  @just build_compiler $(just find_latest_version) current
