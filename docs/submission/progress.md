# Submission progress

Living snapshot of **`main`**. Tracker: [roadmap](roadmap.md).

**Last updated:** 29 May 2026.

## Programme snapshot

| Layer | Status | Issues / PRs |
|-------|--------|--------------|
| **L2** — catalog, Guardian, policies | ✓ Complete | #1–#4, #6 |
| **L3** — lab + smoke + OCSF | **In review** | [#5](https://github.com/edgesentry/agent-control/issues/5), [#7](https://github.com/edgesentry/agent-control/issues/7) · [#31](https://github.com/edgesentry/agent-control/pull/31) merged; trace PR pending |
| **CS01** — SOC + analyst gate | Not started | #9–#10 |

## Shipped (merged to `main`)

| Issue | PR | Deliverable |
|-------|-----|-------------|
| [#1](https://github.com/edgesentry/agent-control/issues/1) | — | Monorepo scaffold |
| [#2](https://github.com/edgesentry/agent-control/issues/2) | [#27](https://github.com/edgesentry/agent-control/pull/27) | Dual license + `cargo-deny` |
| — | [#26](https://github.com/edgesentry/agent-control/pull/26) | MkDocs site |
| [#3](https://github.com/edgesentry/agent-control/issues/3) | [#29](https://github.com/edgesentry/agent-control/pull/29) | OWASP catalog |
| [#4](https://github.com/edgesentry/agent-control/issues/4) | [#28](https://github.com/edgesentry/agent-control/pull/28) | Guardian engine |
| [#6](https://github.com/edgesentry/agent-control/issues/6) | [#30](https://github.com/edgesentry/agent-control/pull/30) | `policies/p0` |
| [#7](https://github.com/edgesentry/agent-control/issues/7) | [#31](https://github.com/edgesentry/agent-control/pull/31) | `apps/lab` + smoke CLI (10/10) |

## In review

| Issue | PR | Deliverable |
|-------|-----|-------------|
| [#5](https://github.com/edgesentry/agent-control/issues/5) | *(this branch)* | **`crates/trace`** — OTel spans → OCSF Detection Finding JSON |

### #5 highlights

| Item | Detail |
|------|--------|
| **`Tracer` + `HookSpan`** | OTel-style in-process spans via `tracing` |
| **`OcsfEvent`** | OCSF class 2004 (Detection Finding) with ACS + OTel in `unmapped` |
| **Lab integration** | `--trace-out` / `AGENT_CONTROL_TRACE_OUT` on smoke |
| **Samples** | `examples/ocsf-deny-tool.json`, `examples/ocsf-events.json` |
| **Tests** | 3 in `trace` + smoke asserts `ocsf_uid` per category |

Closes [#8](https://github.com/edgesentry/agent-control/issues/8) traceability (probe → OCSF).

## Next

| Priority | Issue | Deliverable |
|----------|-------|-------------|
| 1 | [#9–10](https://github.com/edgesentry/agent-control/issues/9) | SOC app + analyst gate |
| 2 | [#11–14](https://github.com/edgesentry/agent-control/issues/11) | Demo script, video, docs polish |
| 3 | [#15](https://github.com/edgesentry/agent-control/issues/15) | Tag `v0.1.0-submission` |

See [success checklist](../plan/success-checklist.md).
