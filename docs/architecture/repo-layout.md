# Repository layout

Crate responsibilities and directory map. Full target tree: [Monorepo layout](../plan/monorepo-layout.md).

## Crate responsibilities

| Crate / app | Challenge | Responsibility |
|-------------|-----------|----------------|
| `guardian` | CS01 + CS02 | Inline hooks; YAML policy evaluation |
| `trace` | CS01 + CS02 | OpenTelemetry spans → OCSF JSON export |
| `lab` | CS02 | CI/CD Observed Agent; drives P0 smoke suite |
| `soc` | CS01 | Alert triage → enrich → recommend (one playbook) |

Stretch: `crates/agbom/` for ASI04 MCP inventory (issue #22).
