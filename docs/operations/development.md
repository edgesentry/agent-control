# Development

## Workspace commands

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
make check   # fmt --check, build, test, clippy, deny
make smoke   # P0 OWASP probes → examples/smoke-report.json
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

Prefix pattern: `AC-{LLM|ASI}{nn}-{slug}` — mapped from `catalog/owasp-llm-asi.yaml` (`crates/catalog` validates at test time).

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

## Dependency policy

`cargo deny check licenses` runs in CI. Allow-list: Apache-2.0, MIT, BSD, ISC, Unicode-3.0, and related permissive licenses — see [deny.toml](../../deny.toml). **GPL / AGPL / LGPL / MPL** are denied for default builds.

```bash
make deny
```

Third-party inventory: [THIRD_PARTY.md](../../THIRD_PARTY.md).
