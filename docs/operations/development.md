# Development

## Workspace commands

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
make check
```

## Conventions

| Topic | Rule |
|-------|------|
| Edition | Rust 2021, `rust-version = "1.75"` in workspace |
| License | `MIT OR Apache-2.0` on workspace crates (files in issue #2) |
| Errors | `thiserror` in libraries; no `unwrap`/`expect` in library code |
| Policies | YAML on **hooks**, not proprietary prompt-only rules |
| Scope | No L1 audit reimplementation; no full SIEM product |

## Smoke test IDs (L3)

Prefix pattern: `AC-{LLM|ASI}{nn}-{slug}` — mapped from `catalog/owasp-llm-asi.yaml` (issue #3).

## Documentation

| Change type | Where to document |
|-------------|-----------------|
| User-facing behaviour | `docs/` (this site) |
| Agent quick reference | `AGENTS.md` |
| Programme milestones | `PLAN.md` |
| One-line human intro | `README.md` |

Build the docs site:

```bash
pip install -r requirements-docs.txt
mkdocs build --strict
mkdocs serve
```

CI publishes to GitHub Pages on changes under `docs/`, `mkdocs.yml`, `README.md`, or `AGENTS.md`.

## Markdown lint

All `**/*.md` files are checked in CI with [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2) (config: `.markdownlint-cli2.jsonc`).

```bash
make md-lint
# or: npx --yes markdownlint-cli2
```

## Dependency policy (issue #2)

Planned CI gate: `cargo deny check licenses` with Apache-2.0, MIT, BSD, ISC allow-list; **exclude GPL/AGPL** from default build.
