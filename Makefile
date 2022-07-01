.PHONY: quartz_test run

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
	cargo run -- run < ./compiler.qz

debug_run:
	@echo "quartz debug_run"
	DEBUG=true cargo run -- run < ./compiler.qz
