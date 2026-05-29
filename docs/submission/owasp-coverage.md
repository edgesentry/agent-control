# OWASP coverage matrix

Submission annex source for CS02: traceability from **OWASP ID → smoke test ID → ACS hook → OCSF event**.

Canonical register: [`catalog/owasp-llm-asi.yaml`](https://github.com/edgesentry/agent-control/blob/main/catalog/owasp-llm-asi.yaml) (issue #3).

Automated probes: `cargo run -p lab -- smoke` ([#31](https://github.com/edgesentry/agent-control/pull/31)). Sample output: [`examples/smoke-report.json`](https://github.com/edgesentry/agent-control/blob/main/examples/smoke-report.json).

## P0 smoke tier (`p0_smoke: true`)

| OWASP ID | Test prefix | Primary hooks | Guardian policy | Smoke probe | OCSF event |
|----------|-------------|---------------|-----------------|-------------|------------|
| LLM01:2025 | AC-LLM01 | `knowledgeRetrieval` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| LLM02:2025 | AC-LLM02 | `agentResponse` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| LLM06:2025 | AC-LLM06 | `toolCallRequest` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| LLM10:2025 | AC-LLM10 | `toolCallRequest`, `agentTrigger` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| ASI01:2026 | AC-ASI01 | `knowledgeRetrieval` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| ASI02:2026 | AC-ASI02 | `toolCallRequest` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| ASI04:2026 | AC-ASI04 | `agbom`, `toolCallRequest` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| ASI05:2026 | AC-ASI05 | `toolCallRequest` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| ASI06:2026 | AC-ASI06 | `memoryStore` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| ASI07:2026 | AC-ASI07 | `a2a`, `agentTrigger` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| ASI08:2026 | AC-ASI08 | `trace` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| ASI09:2026 | AC-ASI09 | `agentResponse`, `humanGate` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |
| ASI10:2026 | AC-ASI10 | `agentTrigger` | ✓ | ✓ [#31](https://github.com/edgesentry/agent-control/pull/31) | #5 |

Full LLM01–10 and ASI01–10 entries (including non-P0) live in the YAML catalog.

Probe definitions: [P0 smoke suite](../plan/p0-smoke-suite.md).
