# Demo script

Live demo for Cap Vista evaluators — target **12–15 minutes** (issue [#14](https://github.com/edgesentry/agent-control/issues/14)).

## Outline (planned)

1. **Problem** (1 min) — Rule-based AppSec cannot test agent *actions* at tool/MCP boundaries; L1 (IoT) vs L2 (agent) separation.
2. **Architecture** (2 min) — `edgesentry-rs` → OWASP register → Guardian → OCSF evidence.
3. **CS02 live** (6 min) — Smoke suite on `apps/lab` on-prem; deny on ASI02 recursion; open OCSF export.
4. **CS01 live** (4 min) — Sample alert to `apps/soc`; destructive tool denied; analyst token → allow.
5. **OSS + scale** (2 min) — ACS, OWASP, OTel, OCSF; Apache-2.0 OR MIT; on-prem bundle.

Recorded video URL for the portal: issue [#17](https://github.com/edgesentry/agent-control/issues/17).
