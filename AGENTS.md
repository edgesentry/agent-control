# AGENTS

High-level guide for coding agents working in **agent-control**. Humans: start with [README.md](README.md). Detail: [docs site](https://edgesentry.github.io/agent-control/) (MkDocs).

## Mission

ACS-aligned **runtime control plane for AI agents** — Cap Vista **CS02** (adversarial security lab) primary, **CS01** (agentic SOC) secondary. **On-prem / air-gap first**; no cloud dependency for core guarantees.

This repo is an **ACS reference implementation**, not the ACS spec. Spec authority: [Agent-Control-Standard/ACS](https://github.com/Agent-Control-Standard/ACS).

## Scope — do not cross

| In scope | Out of scope (other repos) |
|----------|----------------------------|
| `crates/guardian` — hooks, YAML policies | IoT audit chain, sensor profiles → **`edgesentry-rs`** |
| `crates/trace` — OTel → OCSF | Full SIEM/SOAR platform |
| `catalog/` — OWASP LLM01–10, ASI01–10 register | CV / physics / clarus thresholds |
| `apps/lab`, `apps/soc` — Observed Agents | Maritime SBOM (`catena`) |
| `policies/`, `deploy/on-prem/` — L3 harness | Reimplementing L1 in this repo |

**Layers:** L1 = edgesentry-rs · L2 = OWASP + ACS (here) · L3 = smoke IDs, apps, packaging (here when L2 silent).

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
| `PLAN.md` | Submission schedule & DoD (root; not in MkDocs `docs_dir`) |

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
- Smoke test IDs: `AC-{LLM|ASI}{nn}-{slug}` (L3 — document in `docs/` if non-obvious)

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

Do not expand scope into full SOC platform, cloud-only SaaS, or L1 audit chain.

## Documentation layout

| Audience | Location |
|----------|----------|
| Humans (overview) | `README.md` |
| Agents (this file) | `AGENTS.md` |
| Published detail | `docs/` → GitHub Pages via `mkdocs.yml` |
| Internal programme plan | `PLAN.md` (repo root) |

When adding behaviour docs, prefer `docs/` and link from README — keep README and AGENTS.md short.

## ACS hook subset (submission)

Implement in `guardian` unless noted stub-only:

`agentTrigger`, `toolCallRequest`, `agentResponse`, `knowledgeRetrieval`, `memoryStore` — CS02 lab.  
`toolCallResult`, `userMessage` — CS01 soc (issue #20).  
A2A / MCP extensions — stub + doc only before 30 Jun.

## References

- [PLAN.md](PLAN.md) — milestones & acceptance
- [docs/index.md](docs/index.md) — documentation home
- [OWASP Gen AI](https://genai.owasp.org/) · [ACS](https://agentcontrolstandard.ai)
