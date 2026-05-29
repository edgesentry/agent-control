# Success checklist (30 Jun)

## Repo (`agent-control`)

- [ ] Public README with ACS disclaimer, **L1/L2 boundary**, OSS stack table, and on-prem quickstart
- [x] `LICENSE-APACHE`, `LICENSE-MIT`, and `THIRD_PARTY` (or `deny.toml`) committed
- [x] `cargo deny check licenses` passing in CI
- [x] `catalog/owasp-llm-asi.yaml` complete ([#29](https://github.com/edgesentry/agent-control/pull/29))
- [x] Guardian: ≥3 hooks, YAML policies ([#28](https://github.com/edgesentry/agent-control/pull/28))
- [x] `policies/p0/` — OWASP-tagged portable YAML pack ([#30](https://github.com/edgesentry/agent-control/pull/30))
- [x] `apps/lab`: smoke suite 10/10 green ([#31](https://github.com/edgesentry/agent-control/pull/31), in review)
- [ ] `examples/ocsf-*.json` committed (#5)
- [x] OWASP coverage matrix — probe column ([#31](https://github.com/edgesentry/agent-control/pull/31)); OCSF column pending #5
- [ ] `apps/soc`: triage + analyst gate demo
- [ ] Demo script + recorded video
- [ ] Tag `v0.1.0-submission`

Definition of done: [Submission DoD](submission-dod.md).

## Portal

- [ ] CS01 + CS02 selected (or primary + secondary stated)
- [ ] CoC annex PDF uploaded
- [ ] Metrics table = challenge KPIs verbatim
- [ ] Phase 1 resources-from-Organiser section complete
- [ ] “Not looking for” exclusions addressed explicitly
- [ ] Link to `github.com/edgesentry/agent-control`
