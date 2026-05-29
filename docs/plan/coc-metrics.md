# CoC annex metrics (copy-ready)

## CS02 (Phase 1 trial targets)

| Cap Vista metric | Measurement |
|------------------|-------------|
| Attack coverage (OWASP LLM / Agentic) | P0 smoke 10/10 + coverage matrix |
| Accuracy (FP/TP) | Labelled corpus N≥50; methodology in trial report |
| Latency | Smoke suite wall-clock; Guardian p95 per hook |
| Setup / human involvement | One-command smoke; human token only for ASI09 cases |
| SIEM integrability | OCSF JSON export + ingestion screenshot |

## CS01 (Phase 1 trial targets)

| Cap Vista metric | Measurement |
|------------------|-------------|
| Audit trail | % agent actions with OCSF record (target 100%) |
| Human oversight | % destructive actions blocked without token |
| FP rates (AI vs OOTB) | Historical ticket sample; paired comparison design |
| Explainability | Trace replay demo for ≥3 sample alerts |

P0 smoke definitions: [P0 smoke suite](p0-smoke-suite.md).
