.PHONY: build test fmt clippy check

build:
	cargo build --workspace

test:
	cargo test --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

check:
	cargo fmt --all -- --check
	$(MAKE) build test clippy
