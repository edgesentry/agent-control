.PHONY: build test fmt clippy deny check docs-build docs-serve md-lint smoke soc-triage soc-gate

build:
	cargo build --workspace

test:
	cargo test --workspace

smoke:
	cargo run -p lab -- smoke --report examples/smoke-report.json --trace-out examples

soc-triage:
	cargo run -p soc -- triage --report examples/soc-triage-report.json --trace-out examples

soc-gate:
	cargo run -p soc -- gate

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

deny:
	cargo deny check licenses

check:
	cargo fmt --all -- --check
	$(MAKE) build test clippy deny

docs-build:
	mkdocs build --strict

docs-serve:
	mkdocs serve

md-lint:
	npx --yes markdownlint-cli2
