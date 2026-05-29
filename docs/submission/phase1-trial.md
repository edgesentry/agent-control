# Phase 1 trial plan

Post-award trial design for CoC annex (issue [#18](https://github.com/edgesentry/agent-control/issues/18)). **Default deployment: on-prem or air-gapped** via `deploy/on-prem/`.

## CS02 — Guardian Lab (1 month)

| Week | Deliverable |
|------|-------------|
| W1 | Deploy lab + Guardian on evaluator on-prem VM or air-gap kit |
| W2 | OWASP P0 automated tests + coverage PDF |
| W3 | OCSF export to evaluator SIEM (file drop or local forwarder) |
| W4 | Demo + latency report + Phase 2 scale model |

**Ask Organisers:** on-prem VM, SIEM sandbox or file-ingest, optional attack sample pack.

## CS01 — Governed SOC Agent (2 months)

| Week | Deliverable |
|------|-------------|
| W1–2 | One alert source; OOTB triage with ACS trace |
| W3–4 | Guardian on destructive tools; analyst approval UI |
| W5–6 | FP study; audit export; analyst survey |

**Ask Organisers:** on-prem test segment, anonymized alert feed, 2–3 analysts.

## Scheduling

If both challenges are awarded: CS02 weeks 1–4 first; CS01 can start week 2 on a separate config — same Guardian binary, different `apps/*` and `policies/soc/`.

Full detail: [PLAN.md §10](https://github.com/edgesentry/agent-control/blob/main/PLAN.md).
