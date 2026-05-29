# ACS hook subset

ACS is **v0.1 public preview**. Submission and trial commit to this **subset** (see [ACS alignment](../submission/acs-alignment.md) for pinned upstream).

| Hook | CS02 lab | CS01 soc | ACS spec ref |
|------|:--------:|:--------:|--------------|
| `agentTrigger` | ✓ | ✓ | instrument/hooks.md §1 |
| `toolCallRequest` | ✓ | ✓ | §2 |
| `toolCallResult` | — | ✓ | §3 |
| `userMessage` | — | ✓ | §4 |
| `agentResponse` | ✓ | ✓ | §8 |
| `knowledgeRetrieval` | ✓ | — | §6 |
| `memoryStore` | ✓ | — | §7 |
| A2A / MCP extensions | stub or doc | — | Phase 1 stretch |

Guardian evaluation: sync deny on critical tools; trace always async/exported.

Programme context: [MVPs and challenges](../plan/mvps-and-challenges.md) · [Technology choices](../plan/technology.md).
