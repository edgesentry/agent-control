# OWASP risk catalog

Machine-readable register: [`owasp-llm-asi.yaml`](owasp-llm-asi.yaml)

| Field | Meaning |
|-------|---------|
| `id` | OWASP risk id (`LLM01:2025` … `ASI10:2026`) |
| `hooks` | Guardian / ACS Instrument hook ids |
| `acs_methods` | ACS JSON-RPC method names |
| `test_id_prefix` | Smoke probe prefix (`AC-LLM01`, `AC-ASI05`, …) |
| `challenges` | `CS01`, `CS02`, or `both` |
| `p0_smoke` | Included in submission P0 smoke tier |

Load from Rust:

```rust
use catalog::RiskRegister;

let reg = RiskRegister::load_default_repo_file()?;
```

Validator: `cargo test -p catalog`
