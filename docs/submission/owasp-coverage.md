# OWASP coverage matrix

Submission annex source for CS02: traceability from **OWASP ID → smoke test ID → ACS hook → OCSF event**.

Canonical register: [`catalog/owasp-llm-asi.yaml`](https://github.com/edgesentry/agent-control/blob/main/catalog/owasp-llm-asi.yaml).

Automated probes: `cargo run -p lab -- smoke --trace-out examples`. Samples: [`ocsf-deny-tool.json`](https://github.com/edgesentry/agent-control/blob/main/examples/ocsf-deny-tool.json), [`ocsf-events.json`](https://github.com/edgesentry/agent-control/blob/main/examples/ocsf-events.json).

## P0 smoke tier (`p0_smoke: true`)

| OWASP ID | Test prefix | Primary hooks | Guardian policy | Smoke probe | OCSF event |
|----------|-------------|---------------|-----------------|-------------|------------|
| LLM01:2025 | AC-LLM01 | `knowledgeRetrieval` | ✓ | ✓ | ✓ |
| LLM02:2025 | AC-LLM02 | `agentResponse` | ✓ | ✓ | ✓ |
| LLM06:2025 | AC-LLM06 | `toolCallRequest` | ✓ | ✓ | ✓ |
| LLM10:2025 | AC-LLM10 | `toolCallRequest`, `agentTrigger` | ✓ | ✓ | ✓ |
| ASI01:2026 | AC-ASI01 | `knowledgeRetrieval` | ✓ | ✓ | ✓ |
| ASI02:2026 | AC-ASI02 | `toolCallRequest` | ✓ | ✓ | ✓ |
| ASI04:2026 | AC-ASI04 | `agbom`, `toolCallRequest` | ✓ | ✓ | ✓ |
| ASI05:2026 | AC-ASI05 | `toolCallRequest` | ✓ | ✓ | ✓ ([sample](../../examples/ocsf-deny-tool.json)) |
| ASI06:2026 | AC-ASI06 | `memoryStore` | ✓ | ✓ | ✓ |
| ASI07:2026 | AC-ASI07 | `a2a`, `agentTrigger` | ✓ | ✓ | ✓ |
| ASI08:2026 | AC-ASI08 | `trace` | ✓ | ✓ | ✓ |
| ASI09:2026 | AC-ASI09 | `agentResponse`, `humanGate` | ✓ | ✓ | ✓ |
| ASI10:2026 | AC-ASI10 | `agentTrigger` | ✓ | ✓ | ✓ |

Full LLM01–10 and ASI01–10 entries (including non-P0) live in the YAML catalog.

Probe definitions: [P0 smoke suite](../plan/p0-smoke-suite.md). Export: [Trace / OCSF](../architecture/trace.md).
