# agent-control

Open-source **runtime control plane for AI agents** — an [Agent Control Standard (ACS)](https://agentcontrolstandard.ai) aligned reference for Cap Vista **CS02** (security lab) and **CS01** (agentic SOC).

**Submission:** 30 June 2026, 13:00 SGT · [Cap Vista — Cyber Resilience](https://accelerator.capvista.com.sg/en/challenges/solicitation-cyber-resilience-solutions)

Deploy on your own hardware first: no cloud is required to build or run the control plane.

## Documentation

| Audience | Start here |
|----------|------------|
| **Everyone (overview)** | This README |
| **Published detail** | **[https://edgesentry.github.io/agent-control/](https://edgesentry.github.io/agent-control/)** (MkDocs) |
| **Coding agents** | [AGENTS.md](AGENTS.md) → [programme plan](docs/plan/index.md) |
| **Programme plan** | [docs/plan/](docs/plan/index.md) (root `PLAN.md` is a pointer) |

## What it does

- **Guardian** — intercepts agent tool and memory hooks; applies OWASP-tagged YAML policies (`allow` / `deny` / `modify`).
- **Trace** — exports agent steps to **OCSF** for SIEM ingestion.
- **Lab & SOC apps** — demonstration Observed Agents for adversarial testing (CS02) and alert triage (CS01).

IoT and sensor integrity stay in [`edgesentry-rs`](https://github.com/edgesentry/edgesentry-rs); this repo covers **agent** governance only.

## Quick start

```bash
git clone https://github.com/edgesentry/agent-control.git
cd agent-control
cargo build --workspace --release
cargo run -p lab
cargo test --workspace
```

On-prem and air-gapped steps: [docs — Getting started](https://edgesentry.github.io/agent-control/getting-started/) (or `mkdocs serve` locally).

## Status

**L2 foundation (catalog + Guardian + policies) is in place** — P0 smoke automation and trace export are next.

| Area | Status | PR / issue |
|------|--------|------------|
| Monorepo, CI, docs site | ✓ Shipped | [#1](https://github.com/edgesentry/agent-control/issues/1), [#26](https://github.com/edgesentry/agent-control/pull/26) |
| Dual license + `cargo-deny` | ✓ Shipped | [#2](https://github.com/edgesentry/agent-control/issues/2), [#27](https://github.com/edgesentry/agent-control/pull/27) |
| OWASP risk catalog (LLM01–10, ASI01–10) | ✓ Shipped | [#3](https://github.com/edgesentry/agent-control/issues/3), [#29](https://github.com/edgesentry/agent-control/pull/29) |
| Guardian policy engine | ✓ Shipped | [#4](https://github.com/edgesentry/agent-control/issues/4), [#28](https://github.com/edgesentry/agent-control/pull/28) |
| **`policies/p0` OWASP-tagged pack** | **In review** | [#6](https://github.com/edgesentry/agent-control/issues/6), [**#30**](https://github.com/edgesentry/agent-control/pull/30) |
| Trace → OCSF, lab smoke CLI, SOC demo | Planned | [#5](https://github.com/edgesentry/agent-control/issues/5)–[#10](https://github.com/edgesentry/agent-control/issues/10) |

Detail: [submission progress](docs/submission/progress.md) · [issue roadmap](https://edgesentry.github.io/agent-control/submission/roadmap/).

## License

**Apache-2.0 OR MIT** (recipient chooses either):

- [LICENSE-APACHE](LICENSE-APACHE)
- [LICENSE-MIT](LICENSE-MIT)
- Dependency policy: [deny.toml](deny.toml) · [THIRD_PARTY.md](THIRD_PARTY.md) · CI `cargo deny check licenses`

## Links

- [Programme plan](docs/plan/index.md)
- [Agent Control Standard](https://agentcontrolstandard.ai)
- [OWASP Gen AI Security](https://genai.owasp.org/)
- [GitHub — edgesentry/agent-control](https://github.com/edgesentry/agent-control)
