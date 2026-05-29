//! ACS Trace: OpenTelemetry-style spans → OCSF JSON for SIEM export.
//!
//! Records Guardian hook evaluations as spans and maps deny/modify outcomes to
//! OCSF Detection Finding events (class 2004).

mod ocsf;
mod span;

pub use ocsf::{to_json_pretty, write_batch, write_json, OcsfError, OcsfEvent};
pub use span::{GuardianRecord, HookSpan, Tracer};

/// Crate version (matches workspace `0.1.0` until release tagging in issue #15).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Build a [`GuardianRecord`] from hook metadata and a Guardian verdict-shaped payload.
pub fn guardian_record(
    hook: impl Into<String>,
    acs_method: impl Into<String>,
    decision: impl Into<String>,
    message: impl Into<String>,
    reason_codes: Vec<String>,
    matched_policy_ids: Vec<String>,
) -> GuardianRecord {
    GuardianRecord {
        hook: hook.into(),
        acs_method: acs_method.into(),
        decision: decision.into(),
        message: message.into(),
        reason_codes,
        matched_policy_ids,
        session_id: None,
        agent_id: None,
        probe_id: None,
        owasp_ids: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use guardian::{Decision, Guardian, HookRequest};
    use serde_json::json;
    use std::path::PathBuf;

    fn repo_p0() -> Guardian {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../policies/p0");
        Guardian::load_from_dir(&dir).expect("policies/p0")
    }

    #[test]
    fn version_is_non_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn end_to_end_deny_tool_ocsf() {
        let g = repo_p0();
        let params = json!({
            "toolCallRequest": {
                "executionId": "exec-shell",
                "toolId": "shell_exec",
                "inputs": []
            },
            "context": {
                "session": { "id": "lab-session" },
                "agent": { "id": "lab-cicd-agent", "name": "lab" }
            }
        });
        let req = HookRequest::from_jsonrpc("steps/toolCallRequest", params).unwrap();
        let verdict = g.evaluate(&req);
        assert_eq!(verdict.decision, Decision::Deny);

        let mut tracer = Tracer::new();
        let span = tracer.record_guardian(
            "guardian.toolCallRequest",
            GuardianRecord {
                hook: "toolCallRequest".into(),
                acs_method: "steps/toolCallRequest".into(),
                decision: "deny".into(),
                message: verdict.message,
                reason_codes: verdict.reason_codes,
                matched_policy_ids: verdict.matched_policy_ids,
                session_id: Some("lab-session".into()),
                agent_id: Some("lab-cicd-agent".into()),
                probe_id: Some("AC-ASI05-exec".into()),
                owasp_ids: vec!["ASI05:2026".into()],
            },
            1,
        );
        let ocsf = span.to_ocsf();
        assert_eq!(ocsf.class_uid, 2004);
        assert_eq!(ocsf.finding_info.types, vec!["AC-ASI05-exec"]);
    }
}
