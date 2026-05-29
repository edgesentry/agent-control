//! CS01 Governed SOC Agent — alert triage Observed Agent (scaffold).
//!
//! Full playbook: issue #9 (`apps/soc — OOTB alert triage Observed Agent`).

fn main() {
    println!(
        "agent-control soc v{} (guardian {}, trace {})",
        env!("CARGO_PKG_VERSION"),
        guardian::VERSION,
        trace::VERSION,
    );
    println!("Run `make build` or `cargo build --workspace` to compile the workspace.");
}
