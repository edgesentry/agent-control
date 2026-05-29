# agent-control

Open-source runtime control plane for AI agents — an [Agent Control Standard (ACS)](https://agentcontrolstandard.ai) aligned reference implementation for Cap Vista **CS01** (Agentic SOC) and **CS02** (adversarial AI security testing).

**Programme:** [Cap Vista — Cyber Resilience Solutions](https://accelerator.capvista.com.sg/en/challenges/solicitation-cyber-resilience-solutions) · submission **30 Jun 2026, 13:00 SGT**

> **ACS disclaimer:** This repository implements a subset of ACS for demonstration and trial readiness. The [ACS specification](https://github.com/Agent-Control-Standard/ACS) remains the authority; see `docs/acs-alignment.md` (issue #13).

## Security boundary (L1 / L2 / L3)

| Layer | Owner | This repo |
|-------|-------|-----------|
| **L1** — IoT / edge device security, tamper-evident audit | [`edgesentry-rs`](https://github.com/edgesentry/edgesentry-rs) | Out of scope (cite only) |
| **L2** — LLM & agent risk register, hooks, trace | **OWASP** + **ACS** | `catalog/`, `crates/guardian`, `crates/trace` |
| **L3** — Project harness, policies, lab/SOC apps, packaging | **agent-control** | `apps/`, `policies/`, `deploy/` |

**One line:** *edgesentry-rs seals what the device saw; OWASP names agent risks; ACS enforces and records agent actions; agent-control fills the gaps.*

Full boundary doc: `docs/security-boundary.md` (issue #12).

## Repository layout

```text
agent-control/
├── crates/guardian/     # ACS Instrument — hooks + YAML policies
├── crates/trace/        # OTel → OCSF export
├── apps/lab/            # CS02 Observed Agent + smoke CLI
├── apps/soc/            # CS01 alert-triage agent
├── catalog/             # OWASP LLM + Agentic register (YAML)
├── policies/            # OWASP-tagged policy packs
├── examples/            # Sample OCSF / smoke artefacts
├── docs/                # Demo script, coverage matrix, alignment
└── deploy/on-prem/      # Portable lab bundle (Compose / systemd)
```

See [`PLAN.md`](PLAN.md) for the full submission schedule and definition of done.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) **1.75+** (`rustup default stable`)
- `git`, `make` (optional convenience)

No cloud services are required for build or the on-prem quickstart below.

## On-prem quickstart

Run entirely on a local VM, bare metal, or air-gapped host (RPi5-class or evaluator VM).

```bash
# 1. Clone
git clone https://github.com/edgesentry/agent-control.git
cd agent-control

# 2. Build the workspace
cargo build --workspace --release
# or: make build

# 3. Verify binaries
cargo run -p lab
cargo run -p soc

# 4. Run tests (CI-equivalent)
cargo test --workspace
# or: make test
```

Release binaries:

| Binary | Path after release build |
|--------|--------------------------|
| Lab (CS02) | `target/release/lab` |
| SOC (CS01) | `target/release/soc` |

Portable packaging (Docker Compose, systemd) lives under [`deploy/on-prem/`](deploy/on-prem/README.md) (issue #16).

### Air-gapped build

Copy the repository (or a `git archive` tarball) to the isolated host, install Rust offline via [rustup dist files](https://forge.rust-lang.org/infra/other-installation-methods.html#tarballs), then run the same `cargo build --workspace --release` steps. No network is needed after dependencies are vendored (`cargo vendor` — optional, documented in `deploy/on-prem/README.md`).

## Development

```bash
make check    # fmt check (manual: cargo fmt --all -- --check), build, test, clippy
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

CI runs on every push to `main` and on pull requests (`.github/workflows/ci.yml`).

## Roadmap (submission MVP)

| Issue | Deliverable |
|-------|-------------|
| #1 | Monorepo scaffold (this milestone) |
| #2 | Apache-2.0 OR MIT + `cargo-deny` |
| #3–#8 | Catalog, Guardian, trace, policies, lab, smoke suite |
| #9–#10 | SOC agent + analyst approval gate |
| #11–#15 | Docs, demo, `v0.1.0-submission` tag |

## License

Dual-licensed under **Apache-2.0 OR MIT** (recipient chooses either). License files and `cargo-deny` policy: issue #2.

## References

- [PLAN.md](PLAN.md) — Cap Vista submission plan
- [Agent Control Standard](https://agentcontrolstandard.ai)
- [OWASP Gen AI Security Project](https://genai.owasp.org/)
