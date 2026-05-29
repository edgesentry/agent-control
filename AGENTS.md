# AGENTS

High-level guide for coding agents working in **agent-control**. Humans: [README.md](README.md). Detail: [docs site](https://edgesentry.github.io/agent-control/) (MkDocs).

## Mission

ACS-aligned **runtime control plane for AI agents** — Cap Vista **CS02** (adversarial security lab) primary, **CS01** (agentic SOC) secondary. **On-prem / air-gap first**; no cloud dependency for core guarantees.

This repo is an **ACS reference implementation**, not the ACS spec. Spec authority: [Agent-Control-Standard/ACS](https://github.com/Agent-Control-Standard/ACS).

## Programme plan (read first for scope)

Full Cap Vista plan: **[docs/plan/index.md](docs/plan/index.md)** · [Published](https://edgesentry.github.io/agent-control/plan/)

| Topic | Doc |
|-------|-----|
| Purpose, L1/L2/L3, OSS & license | [docs/plan/purpose.md](docs/plan/purpose.md) |
| Submission definition of done | [docs/plan/submission-dod.md](docs/plan/submission-dod.md) |
| P0 smoke suite (10 categories) | [docs/plan/p0-smoke-suite.md](docs/plan/p0-smoke-suite.md) |
| W1–W5 schedule | [docs/plan/schedule.md](docs/plan/schedule.md) |
| Technology & L3 decisions | [docs/plan/technology.md](docs/plan/technology.md) |
| Risks | [docs/plan/risks.md](docs/plan/risks.md) |
| Success checklist (30 Jun) | [docs/plan/success-checklist.md](docs/plan/success-checklist.md) |

Root [PLAN.md](PLAN.md) is a pointer only — **edit `docs/plan/*.md`**.

## Scope — do not cross

| In scope | Out of scope (other repos) |
|----------|----------------------------|
| `crates/guardian` — hooks, YAML policies | IoT audit chain, sensor profiles → **`edgesentry-rs`** |
| `crates/trace` — OTel → OCSF | Full SIEM/SOAR platform |
| `catalog/` — OWASP LLM01–10, ASI01–10 register | CV / physics / clarus thresholds |
| `apps/lab`, `apps/soc` — Observed Agents | Maritime SBOM (`catena`) |
| `policies/`, `deploy/on-prem/` — L3 harness | Reimplementing L1 in this repo |

**Layers:** L1 = edgesentry-rs · L2 = OWASP + ACS (here) · L3 = smoke IDs, apps, packaging (here when L2 silent). Detail: [docs/plan/purpose.md](docs/plan/purpose.md).

## Workspace map

| Path | Role |
|------|------|
| `crates/guardian` | ACS Instrument — `allow` / `deny` / `modify` on hooks |
| `crates/trace` | ACS Trace — OTel spans → OCSF JSON |
| `apps/lab` | CS02 CI/CD agent + `smoke` CLI |
| `apps/soc` | CS01 alert-triage agent (one playbook) |
| `catalog/owasp-llm-asi.yaml` | Risk ID → hooks → test prefixes |
| `policies/p0/`, `policies/soc/` | OWASP-tagged YAML |
| `examples/` | Committed OCSF / smoke report samples |
| `docs/plan/` | Programme plan (canonical) |

Layout: [docs/plan/monorepo-layout.md](docs/plan/monorepo-layout.md).

## Build and verify

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

CI: `.github/workflows/ci.yml`. Docs: `mkdocs build --strict` (see `requirements-docs.txt`).

Before finishing a change: workspace build + tests green; clippy clean; no secrets in tree.

## Conventions

- Rust **2021**, workspace `MIT OR Apache-2.0` (license files: issue #2)
- Match existing crate style; minimal diff; no drive-by refactors
- Policies on **ACS hooks**, not raw prompt strings
- `thiserror` in libraries; avoid `unwrap`/`expect` in library paths
- Smoke test IDs: `AC-{LLM|ASI}{nn}-{slug}` — [docs/plan/technology.md](docs/plan/technology.md)

## GitHub issues (P0 order)

| # | Deliverable |
|---|-------------|
| 1 | Scaffold ✓ |
| 2 | License + `cargo-deny` |
| 3 | `catalog/owasp-llm-asi.yaml` |
| 4 | `crates/guardian` |
| 5 | `crates/trace` |
| 6 | `policies/p0` |
| 7–8 | `apps/lab` + P0 smoke 10/10 |
| 9–10 | `apps/soc` + analyst gate |
| 11–15 | Coverage matrix, docs, demo, tag `v0.1.0-submission` |

Tracker: [docs/submission/roadmap.md](docs/submission/roadmap.md). Do not expand into full SOC platform, cloud-only SaaS, or L1 audit chain.

## Documentation layout

| Audience | Location |
|----------|----------|
| Humans (overview) | `README.md` |
| Agents (this file) | `AGENTS.md` |
| Programme plan | `docs/plan/` |
| Architecture & operations | `docs/architecture/`, `docs/operations/` |
| Published site | GitHub Pages via `mkdocs.yml` |

When adding behaviour docs, prefer `docs/` — keep README and AGENTS.md short.

## ACS hook subset (submission)

| Hook | CS02 | CS01 |
|------|:----:|:----:|
| `agentTrigger`, `toolCallRequest`, `agentResponse` | ✓ | ✓ |
| `knowledgeRetrieval`, `memoryStore` | ✓ | — |
| `toolCallResult`, `userMessage` | — | ✓ (issue #20) |

Full table: [docs/architecture/acs-hooks.md](docs/architecture/acs-hooks.md). A2A / MCP — stub + doc only before 30 Jun.

## References

- [docs/plan/index.md](docs/plan/index.md) — programme plan index
- [docs/index.md](docs/index.md) — documentation home
- [OWASP Gen AI](https://genai.owasp.org/) · [ACS](https://agentcontrolstandard.ai)
