# Third-party licenses

## This repository

Original code and documentation in **agent-control** are licensed under **Apache-2.0 OR MIT** (recipient chooses either). See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).

SPDX expression for workspace crates: `MIT OR Apache-2.0` (see root `Cargo.toml` `[workspace.package]`).

## Rust dependencies (`Cargo.lock`)

Policy: [deny.toml](deny.toml) — **allow-list only** (cargo-deny 0.19+). Permissive licenses are allowed; **GPL / AGPL / LGPL / MPL** and other non-listed licenses fail the check.

| Crate | Version | License | Used by |
|-------|---------|---------|---------|
| *(none)* | — | — | The workspace currently has **no third-party crates** in `Cargo.lock`. |

When dependencies are added, regenerate this table:

```bash
cargo tree -e normal --prefix none | sort -u
cargo deny check licenses
```

## Declared workspace dependencies (not yet in lockfile)

Listed in `[workspace.dependencies]` for upcoming crates (#4–#5). Licenses are expected to pass `cargo deny` when linked:

| Crate | Expected license | Planned use |
|-------|------------------|-------------|
| `serde` | MIT OR Apache-2.0 | Serialization |
| `serde_json` | MIT OR Apache-2.0 | JSON artefacts |
| `thiserror` | MIT OR Apache-2.0 | Error types |
| `tracing` | MIT OR Apache-2.0 | Instrumentation / trace (#5) |

## CI

Every PR and `main` run:

```bash
cargo deny check licenses
```

See [.github/workflows/ci.yml](.github/workflows/ci.yml) job **License check**.
