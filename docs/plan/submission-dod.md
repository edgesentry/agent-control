# Submission definition of done

**Deadline:** 30 June 2026, 13:00 SGT.

Evaluators score **innovativeness**, **feasibility**, and **cost-effective scale** ([timeline & criteria](https://accelerator.capvista.com.sg/en/challenges/solicitation-cyber-resilience-solutions/pages/timeline-evaluation-criteria)). Prototype not mandatory, but **working demo strongly preferred**.

## Must ship in this repo

| # | Deliverable | CS | Acceptance |
|---|-------------|-----|------------|
| 1 | **Monorepo scaffold** — README, **LICENSE-APACHE + LICENSE-MIT**, `THIRD_PARTY`, CI, workspace layout | Both | Public GitHub; **on-prem quickstart** documented; `cargo deny` green |
| 2 | **`catalog/`** — machine-readable OWASP LLM01–10 + ASI01–10 register (YAML) | CS02 | Each ID maps to hook(s) + test ID prefix |
| 3 | **`crates/guardian`** — hook middleware + declarative policies | Both | `allow` / `deny` / `modify` on ≥3 hooks |
| 4 | **`crates/trace`** — OTel spans → OCSF JSON export | Both | Sample export file committed under `examples/` |
| 5 | **`apps/lab`** — CI/CD Observed Agent (coding assistant scenario) | CS02 | Runs smoke tests; Guardian intercepts tool calls |
| 6 | **P0 smoke suite** — 12 automated tests (see [P0 smoke suite](p0-smoke-suite.md)) | CS02 | CLI: `make smoke` or `cargo run -p lab -- smoke` → pass/fail report |
| 7 | **`apps/soc`** — minimal alert-triage Observed Agent (1 playbook) | CS01 | Triage → enrich → recommend; **no** destructive actions without gate |
| 8 | **Analyst approval gate** — human token required before high-impact `allow` | CS01 | Demo: deny without token; allow with token |
| 9 | **Coverage matrix** — OWASP coverage doc filled for P0 tier | CS02 | Traceability: OWASP ID → test ID → hook → OCSF event |
| 10 | **Demo script** — ≤15 minutes live | Both | Recorded video URL for portal (optional but recommended) |
| 11 | **`policies/`** — OWASP-tagged YAML policies (portable, not proprietary DSL) | Both | At least one policy per P0 risk category |

Tracked on GitHub: [issue roadmap](../submission/roadmap.md).

## Must ship in submission pack (edgesentry-commercial)

| Deliverable | Location |
|-------------|----------|
| CoC annex proposal (PDF) | Cap Vista portal — draft from [submission pack](https://github.com/edgesentry/edgesentry-commercial/tree/main/docs/programs/20260630-capvista-cyber-resilience/submission) |
| OWASP coverage annex | [11-owasp-coverage-matrix.md](https://github.com/edgesentry/edgesentry-commercial/blob/main/docs/programs/20260630-capvista-cyber-resilience/submission/11-owasp-coverage-matrix.md) |
| Security boundary | [12-security-boundary.md](https://github.com/edgesentry/edgesentry-commercial/blob/main/docs/programs/20260630-capvista-cyber-resilience/submission/12-security-boundary.md) |
| ACS alignment | [13-acs-alignment.md](https://github.com/edgesentry/edgesentry-commercial/blob/main/docs/programs/20260630-capvista-cyber-resilience/submission/13-acs-alignment.md) |
| Demo script | [14-demo-script.md](https://github.com/edgesentry/edgesentry-commercial/blob/main/docs/programs/20260630-capvista-cyber-resilience/submission/14-demo-script.md) |
| Phase 1 trial + Organiser asks | [15-phase1-trial.md](https://github.com/edgesentry/edgesentry-commercial/blob/main/docs/programs/20260630-capvista-cyber-resilience/submission/15-phase1-trial.md) |
| Metrics table aligned to challenge KPIs | CoC annex ([#175](https://github.com/edgesentry/edgesentry-commercial/issues/175)) |
| Link to agent-control + demo video | CoC annex |

## Explicit non-goals before 30 Jun

- Full LLM01–10 + ASI01–10 regression (Phase 2 scope)
- Production SIEM connector (sample OCSF file is enough for submission)
- **`crates/agbom`** CycloneDX mappers (stretch; stub event OK for submission)
- A2A / MCP full protocol extensions (document as Phase 1 stretch)
- APT detection, weak-signal ML, or full SOC platform (CS01 KPIs beyond ACS layer)
- **IoT / sensor security** — owned by `edgesentry-rs`; cite only as adjacent L1 layer in CoC
- Cloud-only SaaS as the primary deployment story (on-prem lab bundle is the demo default)

Success tracking: [Success checklist](success-checklist.md).
