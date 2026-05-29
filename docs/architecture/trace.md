# Trace (ACS → OCSF)

`crates/trace` records Guardian hook evaluations as OTel-style spans and exports [OCSF Detection Finding](https://schema.ocsf.io/classes/detection_finding) JSON (class **2004**) for SIEM ingest.

## Usage

```rust
use trace::{GuardianRecord, Tracer};

let mut tracer = Tracer::new();
let span = tracer.record_guardian(
    "guardian.toolCallRequest",
    GuardianRecord {
        hook: "toolCallRequest".into(),
        acs_method: "steps/toolCallRequest".into(),
        decision: "deny".into(),
        message: "…".into(),
        reason_codes: vec!["AC-ASI05-exec".into()],
        matched_policy_ids: vec!["p0-deny-shell-exec".into()],
        ..Default::default()
    },
    1,
);
let ocsf = span.to_ocsf();
trace::write_json("examples/ocsf-deny-tool.json", &ocsf)?;
```

Smoke harness (`apps/lab`) emits one OCSF event per P0 probe when `--trace-out` is set:

```bash
cargo run -p lab -- smoke --trace-out examples
make smoke   # also writes examples/ocsf-events.json
```

Environment variable: `AGENT_CONTROL_TRACE_OUT` (see [on-prem deployment](../operations/on-prem.md)).

## Sample artefacts

| File | Description |
|------|-------------|
| [`examples/ocsf-deny-tool.json`](../../examples/ocsf-deny-tool.json) | ASI05 shell/exec deny (single event) |
| [`examples/ocsf-events.json`](../../examples/ocsf-events.json) | Full P0 smoke batch (10 events) |

Guardian verdict, hook, OWASP ids, and OTel trace/span ids are preserved under `unmapped.acs` and `unmapped.otel`.

See [Guardian](guardian.md) · [P0 smoke suite](../plan/p0-smoke-suite.md).
