.PHONY: quartz_test run

test:
	@echo "quartz test"
	ENTRYPOINT=compiler_test cargo run < ./compiler.qz

compile:
	@echo "quartz compile"
	cargo run compile < ./compiler.qz

run:
	@echo "quartz run"
	cargo run < ./compiler.qz

debug_run:
	@echo "quartz debug_run"
	DEBUG=true cargo run < ./compiler.qz
