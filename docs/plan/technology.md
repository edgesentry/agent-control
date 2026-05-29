# Technology choices

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Guardian hot path | **Rust** (`crates/guardian`) | Latency-sensitive inline hooks; same language as `edgesentry-rs` |
| Policy format | **YAML** (+ optional Rego later) | Portable; avoids CS01 “proprietary-only rules” rejection; **L3 project choice** |
| Trace | **OpenTelemetry** SDK → OCSF mapper | ACS trace spec alignment (L2) |
| Lab / SOC agents | **Rust** preferred | Single-language edge/on-prem bundle; Python acceptable for W1 speed only |
| Deployment | **`deploy/on-prem/`** — Compose or systemd | Edge/on-prem first; no cloud required for demo |
| CI | GitHub Actions + **`cargo deny`** (licenses) | Permissive-deps gate; OSS compliance |
| **Product license** | **Apache-2.0 OR MIT** | Dual license; see [Purpose — licensing](purpose.md#product-license-apache-20-or-mit) |

## L3 — project-specific decisions (not mandated by OWASP/ACS)

| Topic | Decision (this repo) |
|-------|----------------------|
| Smoke test ID naming | `AC-{LLM&#124;ASI}{nn}-{slug}` |
| Policy file layout | `policies/p0/`, `policies/soc/` |
| On-prem bundle format | Docker Compose + optional bare-metal systemd units |
| Human approval token | Local HMAC or file-based token (no IdP required for MVP) |
| Default Observed Agent scenarios | CI/CD lab + alert triage (Cap Vista CS02/CS01) |

When OWASP or ACS specifies behaviour, **follow the standard**. When silent, **document the choice** in [Security boundary](../architecture/security-boundary.md).

## Adjacent EdgeSentry assets

| Asset | Layer | Relationship |
|-------|-------|--------------|
| **`edgesentry-rs`** | **L1** | IoT/edge security & tamper-evident records — **not reimplemented here** |
| **`clarus`** | **L1** | Edge collection endpoint; optional upstream of agent context |
| **`agent-control`** | **L2 + L3** | Agent governance (OWASP/ACS) + project-specific harness |
| Port Cyber (Cap Vista Products) | — | Separate programme; AgBOM ≠ shipyard SBOM |
