# Demo script

Live demo for Cap Vista evaluators — target **12–15 minutes** (issue [#14](https://github.com/edgesentry/agent-control/issues/14)).

## Outline

1. **Problem** (1 min) — Rule-based AppSec cannot test agent *actions* at tool/MCP boundaries; IoT integrity (L1) and agent governance (L2) are separate problems.
2. **Architecture** (2 min) — Three layers: `edgesentry-rs` at edge → OWASP register → Guardian hooks → OCSF evidence.
3. **CS02 live** (6 min) — Run smoke suite on `apps/lab` **on-prem**; show deny on ASI02 recursion; open OCSF export.
4. **CS01 live** (4 min) — Feed sample alert to `apps/soc`; show enrichment; attempt destructive tool → deny; approve with token → allow.
5. **OSS + scale** (2 min) — Standards stack (ACS, OWASP, OTel, OCSF); **Apache-2.0 OR MIT**; on-prem / air-gap bundle.

Recorded video for the portal: issue [#17](https://github.com/edgesentry/agent-control/issues/17).

Programme context: [MVPs and challenges](../plan/mvps-and-challenges.md).
