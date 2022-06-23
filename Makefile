.PHONY: quartz_test run

quartz_test:
	@echo "quartz_test"
	ENTRYPOINT=compiler_test cargo run < ./compiler.qz

run:
	@echo "quartz run"
	cargo run < ./compiler.qz

debug_run:
	@echo "quartz debug_run"
	DEBUG=true cargo run < ./compiler.qz
