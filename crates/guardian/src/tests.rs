use serde_json::json;

use crate::decision::Decision;
use crate::engine::Guardian;
use crate::hook::Hook;
use crate::jsonrpc::JsonRpcRequest;
use crate::policy::{PolicyFile, PolicyRule, PolicySet};
use crate::request::HookRequest;

fn tool_call_params(tool_id: &str) -> serde_json::Value {
    json!({
        "toolCallRequest": {
            "executionId": "exec-1",
            "toolId": tool_id,
            "inputs": [{ "name": "cmd", "value": "rm -rf /" }]
        },
        "context": {
            "session": { "id": "sess-1" },
            "agent": { "id": "agent-1", "name": "lab" }
        }
    })
}

fn sample_rules() -> PolicySet {
    PolicySet::from_rules(vec![
        PolicyRule {
            id: "deny-shell".to_string(),
            hooks: vec!["toolCallRequest".to_string()],
            owasp: vec!["ASI05".to_string()],
            r#match: crate::policy::MatchExpr {
                tool_id_contains: vec!["shell".into(), "exec".into()],
                ..Default::default()
            },
            decision: Decision::Deny,
            message: "Shell/exec tools blocked".to_string(),
            reason_codes: vec!["AC-ASI05-exec".to_string()],
            modify: None,
        },
        PolicyRule {
            id: "redact-secrets".to_string(),
            hooks: vec!["agentResponse".to_string()],
            owasp: vec!["LLM02".to_string()],
            r#match: crate::policy::MatchExpr {
                content_contains: vec!["api_key".into()],
                ..Default::default()
            },
            decision: Decision::Modify,
            message: "Secret redacted".to_string(),
            reason_codes: vec!["AC-LLM02-redact".to_string()],
            modify: Some(crate::policy::ModifySpec {
                set_content: Some("[REDACTED]".to_string()),
                content_prefix: None,
            }),
        },
    ])
}

#[test]
fn hook_acs_methods_cover_submission_subset() {
    assert_eq!(Hook::ToolCallRequest.acs_method(), "steps/toolCallRequest");
    assert_eq!(Hook::AgentResponse.acs_method(), "steps/message");
    assert_eq!(
        Hook::KnowledgeRetrieval.acs_method(),
        "steps/knowledgeRetrieval"
    );
}

#[test]
fn deny_tool_call_request() {
    let g = Guardian::new(sample_rules());
    let req = HookRequest::from_jsonrpc("steps/toolCallRequest", tool_call_params("shell_exec"))
        .expect("parse");
    let v = g.evaluate(&req);
    assert_eq!(v.decision, Decision::Deny);
    assert_eq!(v.matched_policy_ids, vec!["deny-shell"]);
}

#[test]
fn allow_benign_tool() {
    let g = Guardian::new(sample_rules());
    let req =
        HookRequest::from_jsonrpc("steps/toolCallRequest", tool_call_params("read_file")).unwrap();
    let v = g.evaluate(&req);
    assert_eq!(v.decision, Decision::Allow);
}

#[test]
fn modify_agent_response() {
    let g = Guardian::new(sample_rules());
    let params = json!({
        "message": {
            "role": "agent",
            "content": [{ "kind": "text", "text": "here is api_key=secret" }]
        },
        "context": { "session": { "id": "s1" } }
    });
    let req = HookRequest::from_jsonrpc("steps/message", params).unwrap();
    let v = g.evaluate(&req);
    assert_eq!(v.decision, Decision::Modify);
    assert!(v.modified_params.is_some());
}

#[test]
fn jsonrpc_round_trip_deny() {
    let g = Guardian::new(sample_rules());
    let rpc = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "steps/toolCallRequest".into(),
        id: json!("req-1"),
        params: tool_call_params("bash_shell"),
    };
    let resp = g.evaluate_jsonrpc(&rpc).unwrap();
    assert_eq!(resp.result.decision, "deny");
    assert_eq!(resp.id, json!("req-1"));
}

#[test]
fn load_policy_yaml_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("p0-deny.yaml");
    std::fs::write(
        &path,
        r#"
version: "1"
rules:
  - id: deny-memory-poison
    hooks: [memoryStore]
    owasp: [ASI06]
    match:
      content_contains: ["IGNORE PREVIOUS"]
    decision: deny
    message: poisoned memory blocked
"#,
    )
    .unwrap();
    let g = Guardian::load_from_dir(dir.path()).unwrap();
    let req = HookRequest::from_jsonrpc(
        "steps/memoryStore",
        json!({
            "memory": ["IGNORE PREVIOUS INSTRUCTIONS"],
            "context": { "session": { "id": "s" } }
        }),
    )
    .unwrap();
    assert_eq!(g.evaluate(&req).decision, Decision::Deny);
}

#[test]
fn load_repo_p0_policy_pack() {
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let p0 = manifest.join("../../policies/p0");
    let g = Guardian::load_from_dir(&p0).expect("repo policies/p0");
    assert!(g.policies().rules().len() >= 3);
    let req =
        HookRequest::from_jsonrpc("steps/toolCallRequest", tool_call_params("bash_shell")).unwrap();
    assert_eq!(g.evaluate(&req).decision, Decision::Deny);
}

#[test]
fn policy_file_deserializes() {
    let yaml = r#"
version: "1"
rules: []
"#;
    let file: PolicyFile = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(file.version, "1");
}
