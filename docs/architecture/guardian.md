# Guardian (ACS Instrument)

`crates/guardian` implements synchronous **allow / deny / modify** decisions on ACS Instrument hooks before an Observed Agent proceeds.

## Hooks (submission subset)

| `Hook` | ACS method |
|--------|------------|
| `agentTrigger` | `steps/agentTrigger` |
| `toolCallRequest` | `steps/toolCallRequest` |
| `agentResponse` | `steps/message` |
| `knowledgeRetrieval` | `steps/knowledgeRetrieval` |
| `memoryStore` | `steps/memoryStore` |
| `agbom` | `inspect/agbom` |
| `a2a` | `a2a/send` |
| `trace` | `trace/correlation` |
| `humanGate` | `instrument/humanGate` |

Policy pack layout: [`policies/p0/README.md`](https://github.com/edgesentry/agent-control/blob/main/policies/p0/README.md).

## Usage

```rust
use guardian::{Guardian, HookRequest, JsonRpcRequest};

let g = Guardian::load_from_dir("policies/p0")?;
let verdict = g.evaluate(&hook_request);

let rpc_response = g.evaluate_jsonrpc(&json_rpc_request)?;
```

Environment variable: `AGENT_CONTROL_POLICY_DIR` (default `./policies/p0`) — used by `apps/lab`.

## Policy YAML (L3)

```yaml
version: "1"
rules:
  - id: p0-deny-shell-exec
    hooks: [toolCallRequest]
    owasp: [ASI05]
    match:
      tool_id_contains: [shell, exec]
    decision: deny
    message: Dangerous tool blocked
    reason_codes: [AC-ASI05-exec]
```

| Field | Purpose |
|-------|---------|
| `hooks` | One or more hook names (camelCase or `steps/...`) |
| `match.tool_id_contains` | Substrings on tool id / inputs |
| `match.content_contains` | Substrings on flattened step text |
| `decision` | `allow`, `deny`, or `modify` |
| `modify.set_content` | Replace message/memory text when modifying |

First matching rule wins; default **allow** if none match.

## ACS response shape

Responses follow [`ACSSuccessResponse`](https://github.com/Agent-Control-Standard/ACS) (`decision`, `message`, optional `modified_request`).

See [ACS hook subset](acs-hooks.md) and [submission DoD](../plan/submission-dod.md).
