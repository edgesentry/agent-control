# ACS hook subset

ACS is **v0.1 public preview**. Submission and Phase 1 trial commit to this **subset** (see [ACS alignment](../submission/acs-alignment.md) for pinned upstream).

| Hook | CS02 lab | CS01 soc | Notes |
|------|:--------:|:--------:|-------|
| `agentTrigger` | ✓ | ✓ | Session / run start |
| `toolCallRequest` | ✓ | ✓ | Primary enforcement point |
| `toolCallResult` | — | ✓ | Issue #20 |
| `userMessage` | — | ✓ | Issue #20 |
| `agentResponse` | ✓ | ✓ | Redaction, overconfidence (ASI09) |
| `knowledgeRetrieval` | ✓ | — | Indirect injection (LLM01 / ASI01) |
| `memoryStore` | ✓ | — | Poisoned memory (ASI06) |
| A2A / MCP extensions | stub | — | Document only before 30 Jun (#23–#24) |

## Evaluation model

- **Guardian:** synchronous `allow` / `deny` / `modify` on critical hooks.
- **Trace:** async export; every decision and agent step should appear in OCSF samples under `examples/`.

Implementation tracker: GitHub issues #4 (guardian), #5 (trace), #20 (extended CS01 hooks).
