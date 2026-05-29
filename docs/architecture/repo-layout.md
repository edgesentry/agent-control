# Repository layout

Target monorepo structure for the Cap Vista submission MVP:

```text
agent-control/
├── AGENTS.md              # Agent-oriented overview (not in this site nav)
├── README.md              # Human-oriented overview
├── PLAN.md                # Programme plan (repo root)
├── mkdocs.yml             # This documentation site
├── catalog/
│   └── owasp-llm-asi.yaml # OWASP LLM01–10 + ASI01–10 → hooks
├── crates/
│   ├── guardian/          # ACS Instrument
│   └── trace/             # OTel → OCSF
├── apps/
│   ├── lab/               # CS02 Observed Agent + smoke CLI
│   └── soc/               # CS01 triage agent
├── policies/
│   ├── p0/                # Smoke-tier YAML (OWASP-tagged)
│   └── soc/               # CS01 production guardrails
├── examples/
│   ├── ocsf-deny-tool.json
│   └── smoke-report.json
├── docs/                  # MkDocs source (published to GitHub Pages)
└── deploy/
    └── on-prem/           # Compose / systemd bundle (issue #16)
```

## Crate responsibilities

| Crate / app | Challenge | Responsibility |
|-------------|-----------|----------------|
| `guardian` | CS01 + CS02 | Inline hooks; YAML policy evaluation |
| `trace` | CS01 + CS02 | OpenTelemetry spans → OCSF JSON export |
| `lab` | CS02 | CI/CD Observed Agent; drives P0 smoke suite |
| `soc` | CS01 | Alert triage → enrich → recommend (one playbook) |

Stretch before or after submission: `crates/agbom/` for ASI04 MCP inventory (issue #22).
