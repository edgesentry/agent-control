# Security boundary

Summary of L1 / L2 / L3. Full narrative: [Purpose and scope](../plan/purpose.md).

EdgeSentry splits security across three layers. **Do not reimplement L1 inside agent-control.**

## Layer model

| Layer | Owner | Artefacts in this repo |
|-------|-------|------------------------|
| **L1** — IoT / edge device security | [`edgesentry-rs`](https://github.com/edgesentry/edgesentry-rs), [`clarus`](https://github.com/edgesentry/clarus) | *Out of scope* |
| **L2** — LLM & agent governance | **OWASP** + **ACS** | `catalog/`, `crates/guardian`, `crates/trace` |
| **L3** — Project harness | **agent-control** | `apps/*`, `policies/`, `deploy/on-prem/` |

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

## L1 ↔ L2 correlation (Phase 2)

Optional: map ACS trace `context.session` or alert IDs to `edgesentry-rs` record hashes when an agent consumes sealed edge data. **Complementary evidence** — not a merged audit chain.
