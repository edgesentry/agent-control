# Monorepo layout (target)

```text
agent-control/
├── AGENTS.md                  # Agent-oriented overview
├── README.md                  # Human-oriented overview
├── PLAN.md                    # Pointer to docs/plan/ (this content)
├── mkdocs.yml
├── catalog/
│   └── owasp-llm-asi.yaml     # LLM01–10 + ASI01–10 → hooks
├── crates/
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
│   ├── plan/                  # Programme plan (this section)
│   ├── architecture/
│   ├── operations/
│   └── submission/
├── LICENSE-APACHE
├── LICENSE-MIT
├── deny.toml                  # cargo-deny: license allow-list
├── THIRD_PARTY.md             # Major OSS deps + licenses
└── deploy/
    └── on-prem/               # Compose / systemd bundle for air-gap trial
```

**Stretch (if time before 30 Jun):** `crates/agbom/` — MCP discovery → JSON inventory snapshot (ASI04 demo).

See also: [Repository layout](../architecture/repo-layout.md).
