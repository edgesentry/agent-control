# Getting started

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) **1.75+** (`rustup default stable`)
- `git` and optionally `make`

No cloud account is required for a local build.

## Clone and build

```bash
git clone https://github.com/edgesentry/agent-control.git
cd agent-control
cargo build --workspace --release
```

Convenience targets:

```bash
make build   # debug build
make test
make check   # fmt --check, build, test, clippy
```

## Run Observed Agent stubs

```bash
cargo run -p lab
cargo run -p soc
```

Release binaries after `cargo build --workspace --release`:

| App | Binary |
|-----|--------|
| CS02 lab | `target/release/lab` |
| CS01 soc | `target/release/soc` |

The smoke CLI (`cargo run -p lab -- smoke`) arrives in issues #7–#8.

## CI parity

Pull requests and `main` run [.github/workflows/ci.yml](https://github.com/edgesentry/agent-control/blob/main/.github/workflows/ci.yml):

- `cargo fmt --all -- --check`
- `cargo build --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`

## Documentation locally

```bash
pip install -r requirements-docs.txt
mkdocs serve
```

Open `http://127.0.0.1:8000`. Published site: <https://edgesentry.github.io/agent-control/>.

## Next reading

- [On-prem deployment](operations/on-prem.md) — air-gapped and portable lab
- [Development](operations/development.md) — conventions and workspace layout
