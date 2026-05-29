# agent-control — Cap Vista Cyber Resilience MVP Plan

**Programme:** [Cap Vista — Solicitation for Cyber Resilience Solutions](https://accelerator.capvista.com.sg/en/challenges/solicitation-cyber-resilience-solutions)  
**Submission deadline:** **30 June 2026, 13:00 SGT (GMT+8)**  
**Repo:** [edgesentry/agent-control](https://github.com/edgesentry/agent-control)  
**Planning date:** 2026-05-29 (updated: deployment & security boundary)  
**Strategy pack (internal):** `edgesentry-commercial/docs/programs/20260630-capvista-cyber-resilience/analytics/`

---

## 1. Purpose

Build an **open-source runtime control plane for AI agents** that supports Cap Vista challenge statements **CS01** (Agentic SOC) and **CS02** (adversarial AI security testing).

**Deployment assumption:** **Edge and on-prem first** — air-gapped or locally hosted Guardian + agents; no cloud dependency for core security guarantees. A portable lab bundle satisfies Cap Vista trial asks (SaaS/on-prem/air-gapped) without reversing this priority.

### 1.1 Security and governance boundary

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

### 1.2 What belongs here vs elsewhere

| In scope (`agent-control`) | Out of scope (other repos) |
|----------------------------|----------------------------|
| Guardian hooks, declarative policies | CV, physics, sensor thresholds (`clarus`, `edgesentry-rs`) |
| OWASP-tagged test catalog & smoke suite | Maritime / port cyber SBOM (`catena`, Products programme) |
| OCSF export for **agent** steps | Deterministic **measurement** audit records (`edgesentry-rs`) |
| Observed Agents (`apps/lab`, `apps/soc`) | Full SIEM/SOAR platform |
| On-prem / air-gap packaging | Cloud-only SaaS as primary product shape |

**Linking L1 ↔ L2 (optional, Phase 2):** correlate ACS trace `context.session` / alert IDs with `edgesentry-rs` record hashes when an agent consumes sealed edge data — complementary evidence, not merged chains.

### 1.3 OSS-first strategy and licensing

**Principle:** Compose from existing open standards and libraries. Implement only the **thin glue** that OWASP, ACS, and the ecosystem do not provide (L3).

#### Maximize OSS — reuse before build

| Layer | Reuse (OSS / open standard) | Build here (minimal) |
|-------|----------------------------|----------------------|
| Risk register | **OWASP** LLM + Agentic Top 10 (public taxonomy) | `catalog/owasp-llm-asi.yaml` — IDs, hook mapping, test refs |
| Hook contract | **ACS** spec + JSON Schema ([Agent-Control-Standard/ACS](https://github.com/Agent-Control-Standard/ACS)) | Guardian adapter + policy evaluation loop |
| Trace | **OpenTelemetry** SDK (Apache-2.0) | ACS attribute mapping; export pipeline |
| SIEM events | **OCSF** schema (open) | OTel span → OCSF JSON mapper (`crates/trace`) |
| AgBOM | **CycloneDX** / **SPDX** formats (open) | Runtime inventory snapshot (stretch) |
| Policies | **YAML**; optional **[Open Policy Agent / Rego](https://www.openpolicyagent.org/)** (Apache-2.0) later | OWASP-tagged policy packs under `policies/` |
| Edge audit (L1) | **`edgesentry-rs`** (EdgeSentry OSS) | Optional correlation IDs only — no reimplementation |
| Runtime / CLI | Rust ecosystem (`tokio`, `serde`, `tracing`, …) | Workspace crates only where no fit |
| Packaging | **Docker Compose**, **systemd** unit templates | `deploy/on-prem/` bundle |

**Do not build:** a custom SIEM, proprietary prompt library, full LLM stack, or IoT audit chain.

**Upstream tracking:** pin ACS commit in `docs/acs-alignment.md`; prefer spec imports over forking ACS Markdown.

#### Product license — Apache-2.0 OR MIT

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

#### Dependency policy (compatible with dual license)

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

#### What remains EdgeSentry-specific (still OSS, Apache-2.0 OR MIT)

- Smoke test harness and IDs (`apps/lab`)
- SOC triage demo agent (`apps/soc`)
- On-prem portable bundle (`deploy/on-prem/`)
- Policy pack layout and Cap Vista coverage matrix tooling

---

## 2. Two MVPs — do not conflate

| Horizon | Audience | Goal |
|---------|----------|------|
| **Submission MVP** (this plan, **by 30 Jun 2026**) | Cap Vista evaluators | Credible OSS repo + live demo + CoC annex evidence that Phase 1 is feasible |
| **Phase 1 trial MVP** (post-award, if shortlisted) | End users in funded trial | CS02: 1 month · CS01: 2 months — **on-prem / portable lab bundle** primary |

Submission MVP **proves readiness**; Phase 1 MVP **executes the trial contract** on evaluator-hosted or air-gapped hardware where required.

## 3. Challenge selection

Apply to **both CS01 and CS02** with one platform story:

| CS | Primary / secondary | Repo focus |
|----|---------------------|------------|
| **CS02** | **Primary** — OWASP coverage is an explicit KPI | `apps/lab`, `catalog/`, Guardian as red-team oracle |
| **CS01** | **Secondary** — same Guardian reused for production SOC guardrails | `apps/soc`, analyst approval gate |

Unified narrative: **CS02 lab proves security; CS01 reuses the same policy pack in operations.**

---

## 4. Submission MVP — definition of done (30 Jun 2026)

Evaluators score **innovativeness**, **feasibility**, and **cost-effective scale** ([timeline & criteria](https://accelerator.capvista.com.sg/en/challenges/solicitation-cyber-resilience-solutions/pages/timeline-evaluation-criteria)). Prototype not mandatory, but **working demo strongly preferred**.

### 4.1 Must ship in this repo

| # | Deliverable | CS | Acceptance |
|---|-------------|-----|------------|
| 1 | **Monorepo scaffold** — README, **LICENSE-APACHE + LICENSE-MIT**, `THIRD_PARTY`, CI, workspace layout | Both | Public GitHub; **on-prem quickstart** documented; `cargo deny` green |
| 2 | **`catalog/`** — machine-readable OWASP LLM01–10 + ASI01–10 register (YAML) | CS02 | Each ID maps to hook(s) + test ID prefix |
| 3 | **`crates/guardian`** (or `packages/guardian`) — hook middleware + declarative policies | Both | `allow` / `deny` / `modify` on ≥3 hooks |
| 4 | **`crates/trace`** — OTel spans → OCSF JSON export | Both | Sample export file committed under `examples/` |
| 5 | **`apps/lab`** — CI/CD Observed Agent (coding assistant scenario) | CS02 | Runs smoke tests; Guardian intercepts tool calls |
| 6 | **P0 smoke suite** — 12 automated tests (see §6) | CS02 | CLI: `make smoke` or `cargo run -p lab -- smoke` → pass/fail report |
| 7 | **`apps/soc`** — minimal alert-triage Observed Agent (1 playbook) | CS01 | Triage → enrich → recommend; **no** destructive actions without gate |
| 8 | **Analyst approval gate** — human token required before high-impact `allow` | CS01 | Demo: deny without token; allow with token |
| 9 | **Coverage matrix** — `docs/owasp-coverage.md` filled for P0 tier | CS02 | Traceability: OWASP ID → test ID → hook → OCSF event |
| 10 | **Demo script** — `docs/demo.md`, ≤15 minutes live | Both | Recorded video URL for portal (optional but recommended) |
| 11 | **`policies/`** — OWASP-tagged YAML policies (portable, not proprietary DSL) | Both | At least one policy per P0 risk category |

### 4.2 Must ship in submission pack (edgesentry-commercial, not this repo)

| Deliverable | Location |
|-------------|----------|
| CoC annex proposal (PDF) | Cap Vista portal |
| Metrics table aligned to challenge KPIs | CoC annex |
| Phase 1 trial design + **resources required from Organisers** | CoC annex |
| OWASP coverage annex (copy from `docs/owasp-coverage.md`) | CoC annex appendix |
| Link to this repo + demo video | CoC annex |

### 4.3 Explicit non-goals before 30 Jun

- Full LLM01–10 + ASI01–10 regression (Phase 2 scope)
- Production SIEM connector (sample OCSF file is enough for submission)
- **`crates/agbom`** CycloneDX mappers (stretch; stub event OK for submission)
- A2A / MCP full protocol extensions (document as Phase 1 stretch)
- APT detection, weak-signal ML, or full SOC platform (CS01 KPIs beyond ACS layer)
- **IoT / sensor security** — owned by `edgesentry-rs`; cite only as adjacent L1 layer in CoC
- Cloud-only SaaS as the primary deployment story (on-prem lab bundle is the demo default)

## 5. Monorepo layout (target)

```text
agent-control/
├── PLAN.md                    ← this file
├── README.md
├── catalog/
│   └── owasp-llm-asi.yaml     # LLM01–10 + ASI01–10 → hooks
├── crates/                    # Rust workspace (preferred for Guardian hot path)
│   ├── guardian/              # Instrument: hooks + policy engine
│   └── trace/                 # OTel → OCSF mapper
├── apps/
│   ├── lab/                   # CS02: CI/CD Observed Agent + test harness
│   └── soc/                   # CS01: alert triage Observed Agent
├── policies/
│   ├── p0/                    # Smoke-tier policies (OWASP-tagged)
│   └── soc/                   # CS01 production guardrails
├── examples/
│   ├── ocsf-deny-tool.json    # Sample SIEM artefact
│   └── smoke-report.json      # Example coverage output
├── docs/
│   ├── demo.md                # 15-min live demo script
│   ├── owasp-coverage.md      # Coverage matrix (submission annex source)
│   ├── acs-alignment.md       # Hook subset vs ACS spec (+ pinned upstream commit)
│   ├── security-boundary.md   # L1 (edgesentry-rs) vs L2 (OWASP/ACS) vs L3 (project)
│   └── phase1-trial.md        # Post-award trial plan (CS01 + CS02)
├── LICENSE-APACHE
├── LICENSE-MIT
├── deny.toml                  # cargo-deny: license allow-list (optional W1)
├── THIRD_PARTY.md             # Major OSS deps + licenses
└── deploy/
    └── on-prem/               # Compose / systemd bundle for air-gap trial
```

**Stretch (if time before 30 Jun):** `crates/agbom/` — MCP discovery → JSON inventory snapshot (ASI04 demo).

---

## 6. P0 smoke suite (CS02 submission bar)

Minimum automated coverage for submission demo and CoC annex. Full mapping: `edgesentry-commercial/.../cap-vista-challenge-acs-mapping.md`.

| ID | Probe (summary) | ACS hook | Pass criteria |
|----|-----------------|----------|---------------|
| LLM01 / ASI01 | Hidden instruction in retrieved doc | `knowledgeRetrieval` | deny or modify + trace |
| LLM02 | Secret exfiltration in response | `agentResponse` | redact + OCSF alert |
| LLM06 / ASI02 | Recursive tool / API calls | `toolCallRequest` | deny after N calls |
| ASI04 | New MCP tool discovered | AgBOM event (or stub) | quarantine policy fires |
| ASI05 | Agent invokes shell / exec | `toolCallRequest` | deny |
| ASI06 | Poisoned memory write | `memoryStore` | deny |
| ASI07 | Unauthorised A2A delegation | A2A hook (or stub + doc) | deny without allowlist |
| ASI08 | Sub-agent cascade failure | trace correlation | cascade pattern in export |
| ASI09 | Overconfident “safe to run” | `agentResponse` + human gate | allow only with analyst token |
| ASI10 / LLM10 | Runaway loop / cost burn | `agentTrigger` + rate limit | deny |

**Submission metric (proposed for CoC):** P0 smoke **10/10 categories** automated with pass/fail + JSON report; latency target **<15 min** wall-clock on **on-prem lab hardware** (RPi5-class or evaluator VM, no cloud dependency).

---

## 7. CS01 submission slice (`apps/soc`)

One OOTB playbook only — **alert triage → enrich → recommend** (no autonomous remediation).

```text
SIEM/SOAR alert → agentTrigger → toolCallRequest(enrichment API) → agentResponse(summary)
                              ↑ Guardian deny/modify
                              ↓ OCSF trace → examples/
```

| CS01 requirement | Submission MVP |
|------------------|----------------|
| OOTB SOC agent | `apps/soc` single playbook |
| Sensor integration | ≥1 mock or real log source (JSON fixture acceptable for submission) |
| Audit trail | 100% of agent actions in OCSF export sample |
| Human oversight | Analyst token gate on destructive tool names (config list) |
| Explainability | Trace includes tool list + Guardian verdict reason |
| Not blackbox | Policies in `policies/soc/` (YAML, OSS) |

**Do not claim:** APT coverage %, blind-spot campaign detection, or analyst productivity gains without labelled trial data — describe **measurement method** in CoC for Phase 1.

---

## 8. ACS hook subset (Phase 1 honesty)

ACS is **v0.1 public preview**. Submission and trial commit to this **subset**:

| Hook | CS02 lab | CS01 soc | ACS spec ref |
|------|----------|----------|--------------|
| `agentTrigger` | ✓ | ✓ | instrument/hooks.md §1 |
| `toolCallRequest` | ✓ | ✓ | §2 |
| `toolCallResult` | — | ✓ | §3 |
| `userMessage` | — | ✓ | §4 |
| `agentResponse` | ✓ | ✓ | §8 |
| `knowledgeRetrieval` | ✓ | — | §6 |
| `memoryStore` | ✓ | — | §7 |
| A2A / MCP extensions | stub or doc | — | Phase 1 stretch |

Guardian evaluation: sync deny on critical tools; trace always async/exported.

---

## 9. Schedule — submission MVP (29 May → 30 Jun 2026)

~32 calendar days. Assume 1 primary engineer; adjust if parallelised.

| Week | Dates | Focus | Exit criteria |
|------|-------|-------|---------------|
| **W1** | 29 May – 4 Jun | Scaffold + Guardian core | Repo public; 3 hooks wired; 1 policy loads |
| **W2** | 5 – 11 Jun | `apps/lab` + catalog + P0 tests (6/10) | CI/CD agent demo; half smoke suite green |
| **W3** | 12 – 18 Jun | trace/OCSF + remaining P0 + `apps/soc` skeleton | OCSF sample; lab smoke 10/10; soc triage path |
| **W4** | 19 – 25 Jun | SOC gate + docs + demo hardening | Analyst gate demo; `docs/demo.md`; coverage matrix complete |
| **W5** | 26 – 30 Jun | Video + CoC annex sync + buffer | Portal PDF uploaded; repo tag `v0.1.0-submission` |

**Hard stop 28 Jun:** feature freeze; 29–30 Jun submission and portal upload only.

---

## 10. Phase 1 trial plan (post-award — for CoC annex)

If shortlisted, trial deliverables differ by challenge duration. **Default deployment: on-prem or air-gapped portable bundle** (`deploy/on-prem/`). SaaS tenant acceptable if Organiser requires it — same binaries, different config.

### CS02 — Guardian Lab (1 month, on-prem primary)

| Week | Deliverable |
|------|-------------|
| W1 | Deploy lab + Guardian on `toolCallRequest`, `agentResponse`, `memoryStore` on evaluator **on-prem VM or air-gap kit** |
| W2 | OWASP P0 automated tests + coverage PDF |
| W3 | OCSF export to evaluator SIEM (local forwarder or file drop); FP/TP on provided corpus (N≥50) |
| W4 | Demo + latency report + Phase 2 scale/cost model |

**Ask Organisers:** On-prem VM or hardware, SIEM sandbox endpoint (or file-ingest path), optional attack sample pack.

### CS01 — Governed SOC Agent (2 months, on-prem SOC segment)

| Week | Deliverable |
|------|-------------|
| W1–2 | Integrate 1 alert source; OOTB triage agent with ACS trace (local SIEM/SOAR or fixture) |
| W3–4 | Guardian on destructive tools; analyst approval UI (runs on-prem) |
| W5–6 | FP study vs OOTB rules; audit export; analyst productivity survey |

**Ask Organisers:** On-prem test segment, anonymized alert feed, 2–3 analyst users.

### Unified scheduling note

CS02 Phase 1 (1 mo) and CS01 Phase 1 (2 mo) overlap if both awarded — same Guardian binary, different `apps/*` configs. Propose **CS02 weeks 1–4 first**, CS01 starts week 2 in parallel on separate tenant.

---

## 11. CoC annex metrics (copy-ready)

### CS02 (Phase 1 trial targets)

| Cap Vista metric | Measurement |
|------------------|-------------|
| Attack coverage (OWASP LLM / Agentic) | P0 smoke 10/10 + matrix in `docs/owasp-coverage.md` |
| Accuracy (FP/TP) | Labelled corpus N≥50; methodology in trial report |
| Latency | Smoke suite wall-clock; Guardian p95 per hook |
| Setup / human involvement | One-command smoke; human token only for ASI09 cases |
| SIEM integrability | OCSF JSON export + ingestion screenshot |

### CS01 (Phase 1 trial targets)

| Cap Vista metric | Measurement |
|------------------|-------------|
| Audit trail | % agent actions with OCSF record (target 100%) |
| Human oversight | % destructive actions blocked without token |
| FP rates (AI vs OOTB) | Historical ticket sample; paired comparison design |
| Explainability | Trace replay demo for ≥3 sample alerts |

---

## 12. Technology choices

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Guardian hot path | **Rust** (`crates/guardian`) | Latency-sensitive inline hooks; same language as `edgesentry-rs` |
| Policy format | **YAML** (+ optional Rego later) | Portable; avoids CS01 “proprietary-only rules” rejection; **L3 project choice** |
| Trace | **OpenTelemetry** SDK → OCSF mapper | ACS trace spec alignment (L2) |
| Lab / SOC agents | **Rust** preferred | Single-language edge/on-prem bundle; Python acceptable for W1 speed only |
| Deployment | **`deploy/on-prem/`** — Compose or systemd | Edge/on-prem first; no cloud required for demo |
| CI | GitHub Actions + **`cargo deny`** (licenses) | Permissive-deps gate; OSS compliance |
| **Product license** | **Apache-2.0 OR MIT** | Dual license; see §1.3 |

### L3 — project-specific decisions (not mandated by OWASP/ACS)

| Topic | Decision (this repo) |
|-------|----------------------|
| Smoke test ID naming | `AC-{LLM\|ASI}{nn}-{slug}` |
| Policy file layout | `policies/p0/`, `policies/soc/` |
| On-prem bundle format | Docker Compose + optional bare-metal systemd units |
| Human approval token | Local HMAC or file-based token (no IdP required for MVP) |
| Default Observed Agent scenarios | CI/CD lab + alert triage (Cap Vista CS02/CS01) |

When OWASP or ACS specifies behaviour, **follow the standard**. When silent, **document the choice in `docs/security-boundary.md`**.

### Adjacent EdgeSentry assets

| Asset | Layer | Relationship |
|-------|-------|--------------|
| **`edgesentry-rs`** | **L1** | IoT/edge security & tamper-evident records — **not reimplemented here** |
| **`clarus`** | **L1** | Edge collection endpoint; optional upstream of agent context |
| **`agent-control`** | **L2 + L3** | Agent governance (OWASP/ACS) + project-specific harness |
| Port Cyber (Cap Vista Products) | — | Separate programme; AgBOM ≠ shipyard SBOM |

---

## 13. Demo script outline (`docs/demo.md`)

**Duration:** 12–15 minutes.

1. **Problem** (1 min) — Rule-based AppSec cannot test agent *actions* at tool/MCP boundaries; IoT integrity (L1) and agent governance (L2) are separate problems.
2. **Architecture** (2 min) — Three layers: `edgesentry-rs` at edge → OWASP register → Guardian hooks → OCSF evidence.
3. **CS02 live** (6 min) — Run smoke suite on `apps/lab` **on-prem**; show deny on ASI02 recursion; open OCSF export.
4. **CS01 live** (4 min) — Feed sample alert to `apps/soc`; show enrichment; attempt destructive tool → deny; approve with token → allow.
5. **OSS + scale** (2 min) — Standards stack (ACS, OWASP, OTel, OCSF); **Apache-2.0 OR MIT**; on-prem / air-gap bundle.

---

## 14. Risks and mitigations

| Risk | Mitigation |
|------|------------|
| ACS v0.1 immaturity | Document hook subset; track upstream ACS v1 |
| Scope creep (full SOC platform) | This repo = control plane only; detection logic out of scope |
| CS01 + CS02 overload by 30 Jun | CS02 lab + smoke is P0; SOC is minimal single-playbook |
| “Indistinguishable from OSS prompt lists” | Policies on **hooks**, not completion strings; publish test IDs |
| Guardian latency | Report p95 in smoke output; async trace |
| Evaluator SIEM unknown | Ship OCSF file; ask Organisers OCSF vs OTel-only in application |
| Dependency license drift | `cargo deny` in CI; `THIRD_PARTY` in repo |
| Blurring L1/L2 scope | Keep IoT/audit chain in `edgesentry-rs`; agent hooks only in this repo |

---

## 15. Success checklist (30 Jun)

### Repo (`agent-control`)

- [ ] Public README with ACS disclaimer, **L1/L2 boundary**, OSS stack table, and on-prem quickstart
- [ ] `LICENSE-APACHE`, `LICENSE-MIT`, and `THIRD_PARTY` (or `deny.toml`) committed
- [ ] `cargo deny check licenses` passing in CI
- [ ] `catalog/owasp-llm-asi.yaml` complete
- [ ] Guardian: ≥3 hooks, YAML policies
- [ ] `apps/lab`: smoke suite 10/10 green
- [ ] `apps/soc`: triage + analyst gate demo
- [ ] `examples/ocsf-*.json` committed
- [ ] `docs/owasp-coverage.md` filled (P0 tier)
- [ ] `docs/demo.md` + recorded video
- [ ] Tag `v0.1.0-submission`

### Portal

- [ ] CS01 + CS02 selected (or primary + secondary stated)
- [ ] CoC annex PDF uploaded
- [ ] Metrics table = challenge KPIs verbatim
- [ ] Phase 1 resources-from-Organiser section complete
- [ ] “Not looking for” exclusions addressed explicitly
- [ ] Link to `github.com/edgesentry/agent-control`

---

## 16. References

- Cap Vista programme index: `edgesentry-commercial/docs/programs/20260630-capvista-cyber-resilience/index.md`
- Sponsor expectations: `.../analytics/sponsor-expectations-and-mvp-proposal-design.md`
- OWASP mapping: `.../analytics/cap-vista-challenge-owasp-mapping.md`
- ACS analysis: `.../analytics/acs-cap-vista-challenge-analysis.md`
- ACS spec mirror: `.../raw/acs/`
- [Agent Control Standard](https://agentcontrolstandard.ai)
- [OWASP Gen AI Security Project](https://genai.owasp.org/)

---

Internal owner: EdgeSentry · Programme: Cap Vista Cyber Resilience · Submission: 30 Jun 2026
