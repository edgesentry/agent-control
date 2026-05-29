# Submission progress

Living snapshot of **`main`**. Tracker: [roadmap](roadmap.md).

**Last updated:** 29 May 2026.

## Shipped

| Issue | PR | Deliverable |
|-------|-----|-------------|
| [#1](https://github.com/edgesentry/agent-control/issues/1) | — | Monorepo scaffold |
| [#2](https://github.com/edgesentry/agent-control/issues/2) | [#27](https://github.com/edgesentry/agent-control/pull/27) | Dual license + `cargo-deny` |
| — | [#26](https://github.com/edgesentry/agent-control/pull/26) | Docs site + `AGENTS.md` |
| [#3](https://github.com/edgesentry/agent-control/issues/3) | [#29](https://github.com/edgesentry/agent-control/pull/29) | OWASP catalog + `crates/catalog` |
| [#4](https://github.com/edgesentry/agent-control/issues/4) | [#28](https://github.com/edgesentry/agent-control/pull/28) | Guardian engine |
| [#6](https://github.com/edgesentry/agent-control/issues/6) | [#30](https://github.com/edgesentry/agent-control/pull/30) | `policies/p0` OWASP pack |

## In review

| Issue | PR | Deliverable |
|-------|-----|-------------|
| [#7](https://github.com/edgesentry/agent-control/issues/7) | *(this branch)* | **`apps/lab`** — CI/CD Observed Agent + `smoke` CLI; 10/10 P0 probes; CI smoke step |

### #7 highlights

- `LabAgent::invoke_tool` — Guardian intercepts `toolCallRequest`
- `cargo run -p lab -- smoke` / `make smoke` — JSON report (`examples/smoke-report.json`)
- 3 unit tests in `apps/lab` (agent + smoke 10/10)

## Next

| Issue | Deliverable |
|-------|-------------|
| [#8](https://github.com/edgesentry/agent-control/issues/8) | Close smoke DoD — OCSF event linkage (#5) |
| [#5](https://github.com/edgesentry/agent-control/issues/5) | `crates/trace` → OCSF export |
| [#9–10](https://github.com/edgesentry/agent-control/issues/9) | SOC app + analyst gate |

See [success checklist](../plan/success-checklist.md).
