# On-prem deployment

**Edge and on-prem first** is the default product shape: Guardian and Observed Agents run on evaluator hardware without a cloud control plane.

## Bare metal / VM quickstart

From the repository root:

```bash
cargo build --workspace --release
./target/release/lab
./target/release/soc
```

Policy packs load from `policies/p0/` and `policies/soc/` relative to the working directory (`policies/p0/README.md`).

## Planned packaging (issue #16)

| Artefact | Purpose |
|----------|---------|
| `docker-compose.yml` | Lab + SOC + optional local SIEM file drop |
| `systemd/` unit templates | Bare-metal / RPi5-class installs |

Source stub: [`deploy/on-prem/README.md`](https://github.com/edgesentry/agent-control/blob/main/deploy/on-prem/README.md).

## Air-gapped builds

On a connected machine:

```bash
cargo vendor vendor/
```

Copy the repo **including** `vendor/` and `.cargo/config.toml` to the isolated host, then:

```bash
cargo build --workspace --release --offline
```

Install Rust offline via [rustup tarball installs](https://forge.rust-lang.org/infra/other-installation-methods.html#tarballs) if needed.

## Environment variables (planned)

| Variable | Default | Purpose |
|----------|---------|---------|
| `AGENT_CONTROL_POLICY_DIR` | `./policies` | YAML policy root |
| `AGENT_CONTROL_TRACE_OUT` | `./examples/` | OCSF export directory (issue #5) |

## Hardware reference

Submission demos target **RPi5-class or evaluator VM** (2+ vCPU, 4 GiB RAM). No GPU required for the ACS control plane.

Phase 1 trial narrative: [Phase 1 trial](../submission/phase1-trial.md).
