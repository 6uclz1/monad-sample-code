CARGO ?= cargo

.PHONY: fmt clippy test check bench run

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

test:
	$(CARGO) test --all-features

check: fmt clippy test

bench:
	$(CARGO) bench --all-features --bench pipeline_bench -- --sample-size 10

run:
	$(CARGO) run -- --in -
