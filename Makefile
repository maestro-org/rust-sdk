BASE_DIR=$(CURDIR)

.PHONY: build format lint test

audit:
	cargo audit

build:
	cargo build --release

format:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test: format
	cargo test --verbose
