# Submission progress

Living snapshot of **`main`** plus open PRs. Tracker: [roadmap](roadmap.md).

**Last updated:** 29 May 2026 (includes [PR #31](https://github.com/edgesentry/agent-control/pull/31) — apps/lab + smoke CLI, in review).

## Programme snapshot

| Layer | Status | Issues / PRs |
|-------|--------|--------------|
| **L2** — catalog, Guardian, policies | ✓ Complete | #1–#4, #6 · [#27](https://github.com/edgesentry/agent-control/pull/27)–[#30](https://github.com/edgesentry/agent-control/pull/30) |
| **L3** — lab harness, smoke IDs | **In review** | [#7](https://github.com/edgesentry/agent-control/issues/7) · [**#31**](https://github.com/edgesentry/agent-control/pull/31) |
| **L3** — trace/OCSF evidence | Not started | [#5](https://github.com/edgesentry/agent-control/issues/5) |
| **CS01** — SOC + analyst gate | Not started | [#9](https://github.com/edgesentry/agent-control/issues/9)–[#10](https://github.com/edgesentry/agent-control/issues/10) |

## Shipped (merged to `main`)

| Issue | PR | Deliverable |
|-------|-----|-------------|
| [#1](https://github.com/edgesentry/agent-control/issues/1) | — | Monorepo scaffold, CI, Makefile |
| [#2](https://github.com/edgesentry/agent-control/issues/2) | [#27](https://github.com/edgesentry/agent-control/pull/27) | Dual license + `cargo-deny` |
| — | [#26](https://github.com/edgesentry/agent-control/pull/26) | MkDocs site, `AGENTS.md`, markdownlint |
| [#3](https://github.com/edgesentry/agent-control/issues/3) | [#29](https://github.com/edgesentry/agent-control/pull/29) | `catalog/owasp-llm-asi.yaml` + `crates/catalog` |
| [#4](https://github.com/edgesentry/agent-control/issues/4) | [#28](https://github.com/edgesentry/agent-control/pull/28) | `crates/guardian` policy engine |
| [#6](https://github.com/edgesentry/agent-control/issues/6) | [#30](https://github.com/edgesentry/agent-control/pull/30) | `policies/p0` — 7 YAML files, 14 OWASP-tagged rules |

## In review

| Issue | PR | Deliverable |
|-------|-----|-------------|
| [#7](https://github.com/edgesentry/agent-control/issues/7) | [**#31**](https://github.com/edgesentry/agent-control/pull/31) | **`apps/lab`** — CI/CD Observed Agent + P0 smoke CLI |

### PR #31 — lab + smoke detail

| Item | Detail |
|------|--------|
| **`LabAgent`** | `invoke_tool` intercepts `toolCallRequest` via Guardian (shell blocked, benign tools allowed) |
| **CLI** | `cargo run -p lab` (demo) · `cargo run -p lab -- smoke` · `make smoke` |
| **P0 probes** | 10 categories — LLM01/ASI01, LLM02, LLM06/ASI02, ASI04–10, LLM10/ASI10 |
| **Report** | JSON to stdout or `--report` (sample: [`examples/smoke-report.json`](../../examples/smoke-report.json)) |
| **Tests** | 3 in `apps/lab` — agent intercept + `p0_smoke_ten_of_ten` |
| **CI** | `cargo run -p lab -- smoke` in [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) |

### #8 partial (same PR)

Probe automation **10/10 green** in local tests, CLI, and CI. Remaining #8 DoD: **OCSF event** per probe (blocked on [#5](https://github.com/edgesentry/agent-control/issues/5) `crates/trace`).

## Next (critical path)

| Priority | Issue | Deliverable |
|----------|-------|-------------|
| 1 | [#5](https://github.com/edgesentry/agent-control/issues/5) | `crates/trace` → OCSF export; close #8 traceability |
| 2 | [#9–10](https://github.com/edgesentry/agent-control/issues/9) | `apps/soc` + analyst approval gate |
| 3 | [#11–14](https://github.com/edgesentry/agent-control/issues/11) | Coverage annex, demo script, video |
| 4 | [#15](https://github.com/edgesentry/agent-control/issues/15) | Tag `v0.1.0-submission` |

W2 exit (schedule): half smoke suite green — **achieved early** via #31; W3 target is trace + SOC skeleton.

See [success checklist](../plan/success-checklist.md) · [P0 smoke suite](../plan/p0-smoke-suite.md).
