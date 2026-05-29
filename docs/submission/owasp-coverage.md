# OWASP coverage matrix

Submission annex source for CS02: traceability from **OWASP ID → smoke test ID → ACS hook → OCSF event**.

Canonical register: [`catalog/owasp-llm-asi.yaml`](https://github.com/edgesentry/agent-control/blob/main/catalog/owasp-llm-asi.yaml) (issue #3).

## P0 smoke tier (`p0_smoke: true`)

| OWASP ID | Test prefix | Primary hooks | Coverage (submission) |
|----------|-------------|---------------|------------------------|
| LLM01:2025 | AC-LLM01 | `knowledgeRetrieval` | Planned (#8) |
| LLM02:2025 | AC-LLM02 | `agentResponse` | Planned (#8) |
| LLM06:2025 | AC-LLM06 | `toolCallRequest` | Planned (#8) |
| LLM10:2025 | AC-LLM10 | `toolCallRequest`, `agentTrigger` | Planned (#8) |
| ASI01:2026 | AC-ASI01 | `knowledgeRetrieval` | Planned (#8) |
| ASI02:2026 | AC-ASI02 | `toolCallRequest` | Planned (#8) |
| ASI04:2026 | AC-ASI04 | `agbom` (stub) | Planned (#8) |
| ASI05:2026 | AC-ASI05 | `toolCallRequest` | Policy + test (#6, #8) |
| ASI06:2026 | AC-ASI06 | `memoryStore` | Policy + test (#6, #8) |
| ASI07:2026 | AC-ASI07 | `a2a` (stub) | Planned (#8) |
| ASI08:2026 | AC-ASI08 | `trace` | Planned (#5, #8) |
| ASI09:2026 | AC-ASI09 | `agentResponse`, `humanGate` | Planned (#10, #8) |
| ASI10:2026 | AC-ASI10 | `agentTrigger` | Planned (#8) |

Full LLM01–10 and ASI01–10 entries (including non-P0) live in the YAML catalog.

Probe definitions: [P0 smoke suite](../plan/p0-smoke-suite.md).
