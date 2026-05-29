# Phase 1 trial plan (post-award)

For CoC annex. If shortlisted, trial deliverables differ by challenge duration. **Default deployment: on-prem or air-gapped portable bundle** (`deploy/on-prem/`). SaaS tenant acceptable if Organiser requires it — same binaries, different config.

## CS02 — Guardian Lab (1 month, on-prem primary)

| Week | Deliverable |
|------|-------------|
| W1 | Deploy lab + Guardian on `toolCallRequest`, `agentResponse`, `memoryStore` on evaluator **on-prem VM or air-gap kit** |
| W2 | OWASP P0 automated tests + coverage PDF |
| W3 | OCSF export to evaluator SIEM (local forwarder or file drop); FP/TP on provided corpus (N≥50) |
| W4 | Demo + latency report + Phase 2 scale/cost model |

**Ask Organisers:** On-prem VM or hardware, SIEM sandbox endpoint (or file-ingest path), optional attack sample pack.

## CS01 — Governed SOC Agent (2 months, on-prem SOC segment)

| Week | Deliverable |
|------|-------------|
| W1–2 | Integrate 1 alert source; OOTB triage agent with ACS trace (local SIEM/SOAR or fixture) |
| W3–4 | Guardian on destructive tools; analyst approval UI (runs on-prem) |
| W5–6 | FP study vs OOTB rules; audit export; analyst productivity survey |

**Ask Organisers:** On-prem test segment, anonymized alert feed, 2–3 analyst users.

## Unified scheduling note

CS02 Phase 1 (1 mo) and CS01 Phase 1 (2 mo) overlap if both awarded — same Guardian binary, different `apps/*` configs. Propose **CS02 weeks 1–4 first**, CS01 starts week 2 in parallel on separate tenant.

Deployment: [On-prem deployment](../operations/on-prem.md).
