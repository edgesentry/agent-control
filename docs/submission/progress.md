# Submission progress

Living snapshot of what is **merged or in review** on [`main`](https://github.com/edgesentry/agent-control). Full issue tracker: [roadmap](roadmap.md).

**Last updated:** 29 May 2026 (includes [PR #30](https://github.com/edgesentry/agent-control/pull/30) — policies/p0, open).

## Shipped

| Issue | PR | Deliverable | Notes |
|-------|-----|-------------|-------|
| [#1](https://github.com/edgesentry/agent-control/issues/1) | — | Monorepo scaffold | Rust workspace, CI, Makefile, on-prem pointer |
| [#2](https://github.com/edgesentry/agent-control/issues/2) | [#27](https://github.com/edgesentry/agent-control/pull/27) | Dual license + `cargo-deny` | Apache-2.0 OR MIT, `THIRD_PARTY.md` |
| — | [#26](https://github.com/edgesentry/agent-control/pull/26) | Docs site | `AGENTS.md`, MkDocs, GitHub Pages, markdownlint |
| [#3](https://github.com/edgesentry/agent-control/issues/3) | [#29](https://github.com/edgesentry/agent-control/pull/29) | OWASP catalog | `catalog/owasp-llm-asi.yaml` — LLM01–10 + ASI01–10; `crates/catalog` validates at test time |
| [#4](https://github.com/edgesentry/agent-control/issues/4) | [#28](https://github.com/edgesentry/agent-control/pull/28) | Guardian engine | `allow` / `deny` / `modify` on 5 core ACS hooks; YAML policy loader; 8+ unit tests |

## In review

| Issue | PR | Deliverable | Highlights |
|-------|-----|-------------|------------|
| [#6](https://github.com/edgesentry/agent-control/issues/6) | [**#30**](https://github.com/edgesentry/agent-control/pull/30) | **`policies/p0` OWASP pack** | 7 YAML files, 14 rules; every P0 smoke OWASP id + ASI01–10 tagged; hooks extended for `agbom`, `a2a`, `trace`, `humanGate`; catalog coverage tests in `crates/guardian` |

### PR #30 — policy pack detail

| File | OWASP ids | Hook(s) |
|------|-----------|---------|
| `deny-dangerous-tools.yaml` | ASI05, ASI06, LLM02 | `toolCallRequest`, `memoryStore`, `agentResponse` |
| `deny-rag-injection.yaml` | LLM01, ASI01 | `knowledgeRetrieval` |
| `deny-tool-misuse.yaml` | LLM06, ASI02, ASI03, ASI04 | `toolCallRequest`, `agentTrigger` |
| `deny-agbom.yaml` | ASI04 | `agbom` |
| `deny-runaway.yaml` | LLM10, ASI10 | `agentTrigger`, `toolCallRequest` |
| `deny-inter-agent.yaml` | ASI07, ASI08 | `a2a`, `agentTrigger`, `trace` |
| `gate-human-trust.yaml` | ASI09 | `agentResponse`, `humanGate` |

Index: [`policies/p0/README.md`](../../policies/p0/README.md). Traceability: [OWASP coverage matrix](owasp-coverage.md).

## Next (critical path)

| Issue | Deliverable | Depends on |
|-------|-------------|------------|
| [#7](https://github.com/edgesentry/agent-control/issues/7) | `apps/lab` smoke CLI | #6 merged |
| [#8](https://github.com/edgesentry/agent-control/issues/8) | P0 smoke 10/10 automated | #7 |
| [#5](https://github.com/edgesentry/agent-control/issues/5) | `crates/trace` → OCSF export | parallel (W3) |
| [#9–10](https://github.com/edgesentry/agent-control/issues/9) | SOC app + analyst gate | after lab smoke |

## Not started (P0)

Trace export (#5), lab/soc apps (#7–10), filled coverage annex (#11), demo video (#14), tag `v0.1.0-submission` (#15).

See [success checklist](../plan/success-checklist.md) and [submission DoD](../plan/submission-dod.md).
