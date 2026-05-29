//! CS02 Guardian Lab — CI/CD Observed Agent and smoke CLI (scaffold).
//!
//! Smoke suite: issue #8 (`P0 smoke suite — 10/10 OWASP categories automated`).

fn main() {
    println!(
        "agent-control lab v{} (guardian {}, trace {})",
        env!("CARGO_PKG_VERSION"),
        guardian::VERSION,
        trace::VERSION,
    );
    println!("Run `make build` or `cargo build --workspace` to compile the workspace.");
    println!("Smoke CLI will be added in issue #7/#8.");
}
