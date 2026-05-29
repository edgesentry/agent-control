# OWASP coverage matrix

Submission annex source for CS02: traceability from **OWASP ID → smoke test ID → ACS hook → OCSF event**.

!!! warning "Stub"
    Filled in issue [#11](https://github.com/edgesentry/agent-control/issues/11). Until then, use the P0 smoke table in [PLAN.md §6](https://github.com/edgesentry/agent-control/blob/main/PLAN.md).

## P0 categories (target 10/10)

| Category | Probe (summary) | Hook |
|----------|-----------------|------|
| LLM01 / ASI01 | Hidden instruction in retrieved doc | `knowledgeRetrieval` |
| LLM02 | Secret exfiltration in response | `agentResponse` |
| LLM06 / ASI02 | Recursive tool calls | `toolCallRequest` |
| ASI04 | New MCP tool discovered | AgBOM event or stub |
| ASI05 | Shell / exec invocation | `toolCallRequest` |
| ASI06 | Poisoned memory write | `memoryStore` |
| ASI07 | Unauthorised A2A delegation | A2A hook or stub |
| ASI08 | Sub-agent cascade failure | trace correlation |
| ASI09 | Overconfident “safe to run” | `agentResponse` + human gate |
| ASI10 / LLM10 | Runaway loop / cost burn | `agentTrigger` + rate limit |

**Submission metric:** automated pass/fail JSON report; wall-clock **&lt;15 min** on on-prem lab hardware.
