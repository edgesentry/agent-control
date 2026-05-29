# agent-control

Open-source **runtime control plane for AI agents** — an [Agent Control Standard (ACS)](https://agentcontrolstandard.ai) aligned reference for Cap Vista **CS02** (security lab) and **CS01** (agentic SOC).

**Submission:** 30 June 2026, 13:00 SGT · [Cap Vista — Cyber Resilience](https://accelerator.capvista.com.sg/en/challenges/solicitation-cyber-resilience-solutions)

Deploy on your own hardware first: no cloud is required to build or run the control plane.

## Documentation

| Audience | Start here |
|----------|------------|
| **Everyone (overview)** | This README |
| **Published detail** | **[https://edgesentry.github.io/agent-control/](https://edgesentry.github.io/agent-control/)** (MkDocs) |
| **Coding agents** | [AGENTS.md](AGENTS.md) |
| **Programme plan** | [PLAN.md](PLAN.md) |

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

Early scaffold — see [roadmap](https://edgesentry.github.io/agent-control/submission/roadmap/) on the docs site. Issue [#1](https://github.com/edgesentry/agent-control/issues/1) (monorepo) is complete; Guardian, trace, and smoke suite follow in #4–#8.

## License

**Apache-2.0 OR MIT** (recipient chooses either). License files and dependency policy: issue [#2](https://github.com/edgesentry/agent-control/issues/2).

## Links

- [Agent Control Standard](https://agentcontrolstandard.ai)
- [OWASP Gen AI Security](https://genai.owasp.org/)
- [GitHub — edgesentry/agent-control](https://github.com/edgesentry/agent-control)
