# P0 policy pack

Portable ACS Instrument policies for Cap Vista **P0 smoke** tier. Each rule is tagged with OWASP short ids (`LLM01`, `ASI05`, …) aligned to [`catalog/owasp-llm-asi.yaml`](../../catalog/owasp-llm-asi.yaml).

| File | OWASP ids |
|------|-----------|
| `deny-dangerous-tools.yaml` | ASI05, ASI06, LLM02 |
| `deny-rag-injection.yaml` | LLM01, ASI01 |
| `deny-tool-misuse.yaml` | LLM06, ASI02, ASI03, ASI04 |
| `deny-agbom.yaml` | ASI04 |
| `deny-runaway.yaml` | LLM10, ASI10 |
| `deny-inter-agent.yaml` | ASI07, ASI08 |
| `gate-human-trust.yaml` | ASI09 |

Load directory: `Guardian::load_from_dir("policies/p0")` or `AGENT_CONTROL_POLICY_DIR`.

Smoke probes use `reason_codes` matching `AC-{LLM|ASI}{nn}-*` prefixes from the catalog.
