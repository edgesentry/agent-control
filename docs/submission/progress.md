# Submission progress

Living snapshot of **`main`**. Tracker: [roadmap](roadmap.md).

**Last updated:** 29 May 2026.

## Programme snapshot

| Layer | Status | Issues / PRs |
|-------|--------|--------------|
| **L2** — catalog, Guardian, policies | ✓ Complete | #1–#4, #6 |
| **L3 CS02** — lab + smoke + OCSF | ✓ Complete | #5–#8 |
| **CS01** — SOC + analyst gate | **In progress** | #9 shipped (this branch); #10 open |

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
| [#5](https://github.com/edgesentry/agent-control/issues/5) | [#32](https://github.com/edgesentry/agent-control/pull/32) | `crates/trace` — OTel spans → OCSF |
| [#8](https://github.com/edgesentry/agent-control/issues/8) | *(via #32)* | P0 smoke 10/10 + OCSF export |

## In review

| Issue | PR | Deliverable |
|-------|-----|-------------|
| [#9](https://github.com/edgesentry/agent-control/issues/9) | *(this branch)* | **`apps/soc`** — OOTB alert-triage playbook |

### #9 highlights

| Item | Detail |
|------|--------|
| **Playbook** | `agentTrigger` → `toolCallRequest` (enrich) → `agentResponse` (recommend) |
| **Fixture** | `examples/alerts/sample-siem-alert.json` |
| **Policies** | `policies/soc/deny-destructive-remediation.yaml` |
| **OCSF** | 5 events per triage run; `--trace-out examples` → `soc-ocsf-events.json` |
| **Tests** | 3 in `soc` (agent + triage) |

## Next

| Priority | Issue | Deliverable |
|----------|-------|-------------|
| 1 | [#10](https://github.com/edgesentry/agent-control/issues/10) | Analyst approval gate (`humanGate` + token) |
| 2 | [#19](https://github.com/edgesentry/agent-control/issues/19) | Expand `policies/soc` |
| 3 | [#11–14](https://github.com/edgesentry/agent-control/issues/11) | Demo script, video, docs polish |
| 4 | [#15](https://github.com/edgesentry/agent-control/issues/15) | Tag `v0.1.0-submission` |

See [success checklist](../plan/success-checklist.md).
