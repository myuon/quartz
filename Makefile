.PHONY: test test_std compile run debug_run

test:
	@echo "quartz test"
	ENTRYPOINT=compiler_test cargo run -- test < ./compiler.qz

test_std:
	@echo "quartz test_std"
	cargo run -- test

compile:
	@echo "quartz compile"
	cargo run -- compile < ./compiler.qz

run:
	@echo "quartz run"
	cargo build
	time cargo run -- run < ./compiler.qz

profile:
	@echo "quartz run --profile"
	cargo run -- run --profile < ./compiler.qz

debug:
	@echo "quartz debug_run"
	DEBUG=true cargo run -- debug quartz-debugger.json

debugger:
	@echo "quartz debugger"
	cargo run -- debugger quartz-debugger.json
