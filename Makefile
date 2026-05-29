.PHONY: build test fmt clippy check docs-build docs-serve md-lint

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

docs-build:
	mkdocs build --strict

docs-serve:
	mkdocs serve

md-lint:
	npx --yes markdownlint-cli2
