# OWASP coverage matrix

Submission annex source for CS02: traceability from **OWASP ID → smoke test ID → ACS hook → OCSF event**.

!!! info "Implementation"
    Filled in issue [#11](https://github.com/edgesentry/agent-control/issues/11). P0 probes and hooks: [P0 smoke suite](../plan/p0-smoke-suite.md).

## Target traceability columns

| Column | Source |
|--------|--------|
| OWASP ID | `catalog/owasp-llm-asi.yaml` |
| Smoke test ID | `AC-{LLM&#124;ASI}{nn}-{slug}` |
| ACS hook | [ACS hook subset](../architecture/acs-hooks.md) |
| OCSF event | `examples/` + `crates/trace` |

CoC metrics: [CoC metrics](../plan/coc-metrics.md).
