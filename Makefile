.PHONY: quartz_test run

quartz_test:
	@echo "quartz_test"
	ENTRYPOINT=compiler_test cargo run  < ./compiler.qz

run:
	@echo "quartz"
	cargo run  < ./compiler.qz
