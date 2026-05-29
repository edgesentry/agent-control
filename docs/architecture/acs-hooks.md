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
| `agbom` | ✓ (policy) | — | inspect/agbom — [#30](https://github.com/edgesentry/agent-control/pull/30) |
| `a2a` | ✓ (policy) | — | a2a/send — [#30](https://github.com/edgesentry/agent-control/pull/30) |
| `trace` | ✓ (policy) | — | trace/correlation — [#30](https://github.com/edgesentry/agent-control/pull/30) |
| `humanGate` | ✓ (policy) | ✓ (gate) | instrument/humanGate — [#30](https://github.com/edgesentry/agent-control/pull/30) |
| A2A / MCP runtime wiring in lab | planned (#7–8) | — | Phase 1 stretch |

Guardian evaluation: sync deny on critical tools; trace always async/exported.

Programme context: [MVPs and challenges](../plan/mvps-and-challenges.md) · [Technology choices](../plan/technology.md).
