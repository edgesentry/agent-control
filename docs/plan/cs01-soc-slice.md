# CS01 submission slice (`apps/soc`)

One OOTB playbook only — **alert triage → enrich → recommend** (no autonomous remediation).

```text
SIEM/SOAR alert → agentTrigger → toolCallRequest(enrichment API) → agentResponse(summary)
                              ↑ Guardian deny/modify
                              ↓ OCSF trace → examples/
```

| CS01 requirement | Submission MVP |
|------------------|----------------|
| OOTB SOC agent | `apps/soc` single playbook |
| Sensor integration | ≥1 mock or real log source (JSON fixture acceptable for submission) |
| Audit trail | 100% of agent actions in OCSF export sample |
| Human oversight | Analyst token gate on destructive tool names (config list) |
| Explainability | Trace includes tool list + Guardian verdict reason |
| Not blackbox | Policies in `policies/soc/` (YAML, OSS) |

**Do not claim:** APT coverage %, blind-spot campaign detection, or analyst productivity gains without labelled trial data — describe **measurement method** in CoC for Phase 1.

Phase 1 trial: [Phase 1 trial — CS01](phase1-trial.md#cs01-governed-soc-agent-2-months-on-prem-soc-segment).
