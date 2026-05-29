//! CS02 Guardian Lab — CI/CD Observed Agent and smoke CLI (scaffold).
//!
//! Smoke suite: issue #8 (`P0 smoke suite — 10/10 OWASP categories automated`).

use std::env;
use std::path::PathBuf;

use guardian::{Guardian, JsonRpcRequest};
use serde_json::json;

fn main() {
    let policy_dir = env::var("AGENT_CONTROL_POLICY_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("policies/p0"));

    println!(
        "agent-control lab v{} (guardian {})",
        env!("CARGO_PKG_VERSION"),
        guardian::VERSION,
    );

    match Guardian::load_from_dir(&policy_dir) {
        Ok(g) => {
            println!(
                "Loaded {} policy rule(s) from {}",
                g.policies().rules().len(),
                policy_dir.display()
            );
            demo_deny(&g);
        }
        Err(e) => {
            eprintln!("Policy load skipped ({policy_dir:?}): {e}");
        }
    }
}

fn demo_deny(g: &Guardian) {
    let rpc = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "steps/toolCallRequest".into(),
        id: json!("demo-1"),
        params: json!({
            "toolCallRequest": {
                "executionId": "e1",
                "toolId": "shell_exec",
                "inputs": []
            },
            "context": { "session": { "id": "lab" } }
        }),
    };
    if let Ok(resp) = g.evaluate_jsonrpc(&rpc) {
        println!(
            "Demo toolCallRequest → decision={} message={}",
            resp.result.decision, resp.result.message
        );
    }
}
