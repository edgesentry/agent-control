# Risks and mitigations

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
