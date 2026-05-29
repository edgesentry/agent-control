# Security boundary

EdgeSentry splits security across three layers. **Do not reimplement L1 inside agent-control.**

## Layer model

| Layer | Owner | Artefacts in this repo |
|-------|-------|------------------------|
| **L1** — IoT / edge device security | [`edgesentry-rs`](https://github.com/edgesentry/edgesentry-rs), [`clarus`](https://github.com/edgesentry/clarus) at collection edge | *Out of scope* — cite only in CoC / architecture narrative |
| **L2** — LLM & agent governance | **OWASP** + **ACS** | `catalog/`, `crates/guardian`, `crates/trace` |
| **L3** — Project harness | **agent-control** | `apps/*`, `policies/`, smoke IDs, `deploy/on-prem/` |

```text
┌─────────────────────────────────────────────────────────┐
│  L3 — smoke IDs · lab/soc · policy layout · packaging    │
└──────────────────────────┬──────────────────────────────┘
                           │ only where OWASP/ACS are silent
┌──────────────────────────▼──────────────────────────────┐
│  L2 — OWASP register · Guardian · OCSF trace · AgBOM    │
└──────────────────────────┬──────────────────────────────┘
                           │ not duplicated here
┌──────────────────────────▼──────────────────────────────┐
│  L1 — tamper-evident audit · device identity (edgesentry-rs) │
└─────────────────────────────────────────────────────────┘
```

## Concern matrix

| Concern | Owner | Standard / path |
|---------|-------|-----------------|
| Sensor & pipeline integrity | `edgesentry-rs` | BLAKE3 + Ed25519 audit chain |
| LLM & agentic risk register | OWASP | `catalog/owasp-llm-asi.yaml` |
| Runtime allow/deny/modify | ACS Instrument | `crates/guardian` |
| Agent audit to SIEM | ACS Trace | `crates/trace` → OCSF |
| Dynamic tool inventory (stretch) | ACS Inspect | `crates/agbom` (issue #22) |
| Harness & deployment bundle | agent-control L3 | `apps/`, `deploy/on-prem/` |

## L1 ↔ L2 correlation (Phase 2)

Optional: map ACS trace `context.session` or alert IDs to `edgesentry-rs` record hashes when an agent consumes sealed edge data. **Complementary evidence** — not a merged audit chain.

Full programme context: [PLAN.md](https://github.com/edgesentry/agent-control/blob/main/PLAN.md) §1.1–1.2.
