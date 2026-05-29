# Purpose and scope

## Purpose

Build an **open-source runtime control plane for AI agents** that supports Cap Vista challenge statements **CS01** (Agentic SOC) and **CS02** (adversarial AI security testing).

**Deployment assumption:** **Edge and on-prem first** — air-gapped or locally hosted Guardian + agents; no cloud dependency for core security guarantees. A portable lab bundle satisfies Cap Vista trial asks (SaaS/on-prem/air-gapped) without reversing this priority.

## Security and governance boundary

EdgeSentry splits responsibility across three layers. **Do not duplicate** `edgesentry-rs` IoT concerns inside `agent-control`.

```text
┌─────────────────────────────────────────────────────────────────┐
│  L3 — Project-specific (this repo decides)                       │
│  Smoke test IDs · lab/soc apps · policy YAML schema · packaging  │
└────────────────────────────┬────────────────────────────────────┘
                             │ only where OWASP/ACS are silent
┌────────────────────────────▼────────────────────────────────────┐
│  L2 — Agent / LLM security & governance (OWASP + ACS)            │
│  Risk register · Guardian hooks · OCSF trace · AgBOM           │
│  → implemented in agent-control (ACS-aligned reference impl)     │
└────────────────────────────┬────────────────────────────────────┘
                             │ out of scope for edgesentry-rs
┌────────────────────────────▼────────────────────────────────────┐
│  L1 — IoT / edge device security (edgesentry-rs)                 │
│  Tamper-evident audit chain · device/sensor identity · profiles  │
│  → edgesentry/edgesentry-rs (+ clarus at the collection edge)    │
└─────────────────────────────────────────────────────────────────┘
```

| Concern | Owner | Standard / artefact |
|---------|-------|---------------------|
| Sensor & pipeline integrity at the edge | **`edgesentry-rs`** | BLAKE3 + Ed25519 audit chain; domain profiles |
| IoT device security, physical edge collection | **`edgesentry-rs` / clarus** | Not this repo |
| LLM & agentic **risk register** | **OWASP** | `catalog/` — LLM01–10, ASI01–10 |
| Runtime **control** (allow/deny/modify) | **ACS** Instrument | `crates/guardian` |
| Agent action **audit** to SIEM | **ACS** Trace | OTel → OCSF (`crates/trace`) |
| Dynamic agent **inventory** (tools, MCP) | **ACS** Inspect | `crates/agbom` (stretch) |
| Test harness layout, smoke IDs, deployment bundle | **`agent-control`** (L3) | This repo when L1/L2 silent |

**One-line pitch:** *edgesentry-rs seals what the device saw; OWASP names agent risks; ACS enforces and records agent actions; agent-control fills the gaps.*

This repository is an **ACS-aligned reference implementation**, not the ACS specification itself. Spec authority: [Agent-Control-Standard/ACS](https://github.com/Agent-Control-Standard/ACS).

See also: [Security boundary](../architecture/security-boundary.md).

## What belongs here vs elsewhere

| In scope (`agent-control`) | Out of scope (other repos) |
|----------------------------|----------------------------|
| Guardian hooks, declarative policies | CV, physics, sensor thresholds (`clarus`, `edgesentry-rs`) |
| OWASP-tagged test catalog & smoke suite | Maritime / port cyber SBOM (`catena`, Products programme) |
| OCSF export for **agent** steps | Deterministic **measurement** audit records (`edgesentry-rs`) |
| Observed Agents (`apps/lab`, `apps/soc`) | Full SIEM/SOAR platform |
| On-prem / air-gap packaging | Cloud-only SaaS as primary product shape |

**Linking L1 ↔ L2 (optional, Phase 2):** correlate ACS trace `context.session` / alert IDs with `edgesentry-rs` record hashes when an agent consumes sealed edge data — complementary evidence, not merged chains.

## OSS-first strategy and licensing

**Principle:** Compose from existing open standards and libraries. Implement only the **thin glue** that OWASP, ACS, and the ecosystem do not provide (L3).

### Maximize OSS — reuse before build

| Layer | Reuse (OSS / open standard) | Build here (minimal) |
|-------|----------------------------|----------------------|
| Risk register | **OWASP** LLM + Agentic Top 10 (public taxonomy) | `catalog/owasp-llm-asi.yaml` — IDs, hook mapping, test refs |
| Hook contract | **ACS** spec + JSON Schema | Guardian adapter + policy evaluation loop |
| Trace | **OpenTelemetry** SDK (Apache-2.0) | ACS attribute mapping; export pipeline |
| SIEM events | **OCSF** schema (open) | OTel span → OCSF JSON mapper (`crates/trace`) |
| AgBOM | **CycloneDX** / **SPDX** formats (open) | Runtime inventory snapshot (stretch) |
| Policies | **YAML**; optional **OPA / Rego** later | OWASP-tagged policy packs under `policies/` |
| Edge audit (L1) | **`edgesentry-rs`** (EdgeSentry OSS) | Optional correlation IDs only — no reimplementation |
| Runtime / CLI | Rust ecosystem (`tokio`, `serde`, `tracing`, …) | Workspace crates only where no fit |
| Packaging | **Docker Compose**, **systemd** unit templates | `deploy/on-prem/` bundle |

**Do not build:** a custom SIEM, proprietary prompt library, full LLM stack, or IoT audit chain.

**Upstream tracking:** pin ACS commit in [ACS alignment](../submission/acs-alignment.md); prefer spec imports over forking ACS Markdown.

### Product license — Apache-2.0 OR MIT

All **original code and documentation** in this repository is dual-licensed:

```text
SPDX-License-Identifier: Apache-2.0 OR MIT
```

| Artefact | License |
|----------|---------|
| `agent-control` original code & docs | **Apache-2.0 OR MIT** (recipient chooses either) |
| `LICENSE-APACHE` + `LICENSE-MIT` | Standard Rust dual-license file pair at repo root |
| `THIRD_PARTY` / `cargo-deny` allow-list | Documents dependency licenses |

**Rationale:** Apache-2.0 OR MIT is the common Rust OSS pattern (e.g. Tokio, many Cloud Native projects). Defence and enterprise adopters get explicit patent grant (Apache) or minimal MIT terms — their choice.

### Dependency policy (compatible with dual license)

| Allowed | Notes |
|---------|--------|
| Apache-2.0, MIT, BSD-2/3-Clause, ISC, Unicode-3.0 | Default for `cargo deny` / CI |
| ACS (MIT), OpenTelemetry (Apache-2.0), OPA (Apache-2.0) | Expected direct deps |

| Review required | Action |
|-----------------|--------|
| LGPL / MPL (weak copyleft) | Case-by-case; avoid in Guardian hot path |
| GPL / AGPL | **Exclude** from default build |
| Proprietary / unknown | **Exclude** |

CI gate (W1): `cargo deny check licenses` (or equivalent) on every PR.

### What remains EdgeSentry-specific (still OSS, Apache-2.0 OR MIT)

- Smoke test harness and IDs (`apps/lab`)
- SOC triage demo agent (`apps/soc`)
- On-prem portable bundle (`deploy/on-prem/`)
- Policy pack layout and Cap Vista coverage matrix tooling
