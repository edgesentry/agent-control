# ACS alignment

This repository implements a **subset** of the [Agent Control Standard](https://agentcontrolstandard.ai) for Cap Vista demonstration and trial readiness.

!!! note
    Spec authority remains [Agent-Control-Standard/ACS](https://github.com/Agent-Control-Standard/ACS). Issue [#13](https://github.com/edgesentry/agent-control/issues/13) will pin an upstream commit in this document.

## Implemented surfaces (target)

| ACS surface | Crate / path |
|-------------|--------------|
| Instrument (hooks + policies) | `crates/guardian` |
| Trace (OTel → OCSF) | `crates/trace` |
| Inspect (AgBOM, stretch) | `crates/agbom` (issue #22) |

## Hook subset

See [ACS hook subset](../architecture/acs-hooks.md).

## Drift policy

- Prefer importing ACS JSON Schema / hook names from upstream over forking spec Markdown.
- Document any intentional subset or extension in this file before submission tag `v0.1.0-submission`.
