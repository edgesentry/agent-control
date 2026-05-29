# Third-party licenses

## This repository

Original code and documentation in **agent-control** are licensed under **Apache-2.0 OR MIT** (recipient chooses either). See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).

SPDX expression for workspace crates: `MIT OR Apache-2.0` (see root `Cargo.toml` `[workspace.package]`).

## Rust dependencies (`Cargo.lock`)

Policy: [deny.toml](deny.toml) — **allow-list only** (cargo-deny 0.19+). Permissive licenses are allowed; **GPL / AGPL / LGPL / MPL** and other non-listed licenses fail the check.

| Crate | License | Used by |
|-------|---------|---------|
| `serde` | MIT OR Apache-2.0 | `guardian`, `lab` |
| `serde_json` | MIT OR Apache-2.0 | `guardian`, `lab` |
| `serde_yaml` | MIT OR Apache-2.0 | `guardian` (policy packs) |
| `thiserror` | MIT OR Apache-2.0 | `guardian` |
| `indexmap` | MIT OR Apache-2.0 | transitive (`serde_yaml`) |
| `unsafe-libyaml` | MIT | transitive (`serde_yaml`) |

Regenerate after dependency changes:

```bash
cargo deny check licenses
cargo tree -e normal --prefix none
```

## Workspace dependencies (not yet linked)

| Crate | Expected license | Planned use |
|-------|------------------|-------------|
| `tracing` | MIT OR Apache-2.0 | `crates/trace` (#5) |

## CI

Every PR and `main` run:

```bash
cargo deny check licenses
```

See [.github/workflows/ci.yml](.github/workflows/ci.yml) job **License check**.
