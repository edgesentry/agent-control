//! CS01 alert-triage Observed Agent — routes ACS steps through Guardian.

use guardian::{Guardian, HookRequest};
use serde_json::{json, Value};
use thiserror::Error;

/// Minimal SOC agent for the alert-triage playbook.
pub struct SocAgent {
    guardian: Guardian,
    session_id: String,
    agent_id: String,
}

#[derive(Debug, Error)]
pub enum SocError {
    #[error("guardian evaluation failed: {0}")]
    Evaluate(String),
}

impl SocAgent {
    pub fn new(guardian: Guardian) -> Self {
        Self {
            guardian,
            session_id: "soc-triage-session".into(),
            agent_id: "soc-triage-agent".into(),
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    fn context(&self) -> Value {
        json!({
            "session": { "id": self.session_id },
            "agent": { "id": self.agent_id, "name": "soc" }
        })
    }

    /// Evaluate any ACS JSON-RPC step through Guardian.
    pub fn evaluate_step(
        &self,
        method: &str,
        params: Value,
    ) -> Result<guardian::Verdict, SocError> {
        let hook = HookRequest::from_jsonrpc(method, params)
            .map_err(|e| SocError::Evaluate(e.to_string()))?;
        Ok(self.guardian.evaluate(&hook))
    }

    /// `agentTrigger` — SIEM/SOAR alert ingested as playbook entry.
    pub fn on_alert_trigger(&self, alert_summary: &str) -> Result<guardian::Verdict, SocError> {
        let params = json!({
            "trigger": {
                "type": "alert",
                "content": [{ "kind": "text", "text": alert_summary }]
            },
            "context": self.context()
        });
        self.evaluate_step("steps/agentTrigger", params)
    }

    fn tool_call_params(&self, tool_id: &str, inputs: &[(&str, &str)]) -> Value {
        let input_values: Vec<Value> = inputs
            .iter()
            .map(|(name, value)| json!({ "name": name, "value": value }))
            .collect();
        json!({
            "toolCallRequest": {
                "executionId": format!("exec-{tool_id}"),
                "toolId": tool_id,
                "inputs": input_values
            },
            "context": self.context()
        })
    }

    /// Evaluate `toolCallRequest` and return the Guardian verdict.
    pub fn tool_call_verdict(
        &self,
        tool_id: &str,
        inputs: &[(&str, &str)],
    ) -> Result<guardian::Verdict, SocError> {
        self.tool_call_verdict_with_approval(tool_id, inputs, None)
    }

    /// `toolCallRequest` with optional analyst approval token in inputs.
    pub fn tool_call_verdict_with_approval(
        &self,
        tool_id: &str,
        inputs: &[(&str, &str)],
        approval_token: Option<&str>,
    ) -> Result<guardian::Verdict, SocError> {
        let mut all_inputs: Vec<(&str, &str)> = inputs.to_vec();
        if let Some(token) = approval_token {
            all_inputs.push(("analyst_approval_token", token));
        }
        self.evaluate_step(
            "steps/toolCallRequest",
            self.tool_call_params(tool_id, &all_inputs),
        )
    }

    /// `humanGate` — analyst approval before high-impact remediation.
    pub fn human_gate_verdict(
        &self,
        tool_id: &str,
        risk: &str,
        approval_token: Option<&str>,
    ) -> Result<guardian::Verdict, SocError> {
        let params = json!({
            "approval": { "token": approval_token.unwrap_or("") },
            "action": { "risk": risk, "toolId": tool_id },
            "context": self.context()
        });
        self.evaluate_step("instrument/humanGate", params)
    }

    /// `agentResponse` — triage summary returned to analyst.
    pub fn respond(&self, summary: &str) -> Result<guardian::Verdict, SocError> {
        let params = json!({
            "message": {
                "role": "agent",
                "content": [{ "kind": "text", "text": summary }]
            },
            "context": self.context()
        });
        self.evaluate_step("steps/message", params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn soc_agent() -> SocAgent {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../policies/soc");
        let g = Guardian::load_from_dir(&dir).expect("policies/soc");
        SocAgent::new(g)
    }

    use guardian::Decision;

    #[test]
    fn blocks_destructive_isolation_tool() {
        let agent = soc_agent();
        let verdict = agent
            .tool_call_verdict("isolate_host", &[("host", "ws-finance-17")])
            .unwrap();
        assert_eq!(verdict.decision, Decision::Deny);
        assert!(verdict
            .reason_codes
            .contains(&"AC-SOC01-remediation-gate".to_string()));
    }

    #[test]
    fn allows_enrichment_lookup() {
        let agent = soc_agent();
        let verdict = agent
            .tool_call_verdict(
                "lookup_threat_intel",
                &[("ioc_type", "ip"), ("ioc_value", "203.0.113.42")],
            )
            .unwrap();
        assert_eq!(verdict.decision, Decision::Allow);
    }

    #[test]
    fn allows_destructive_tool_with_approval_token() {
        use crate::config::DEFAULT_APPROVAL_TOKEN;

        let agent = soc_agent();
        let verdict = agent
            .tool_call_verdict_with_approval(
                "isolate_host",
                &[("host", "ws-finance-17")],
                Some(DEFAULT_APPROVAL_TOKEN),
            )
            .unwrap();
        assert_eq!(verdict.decision, Decision::Allow);
    }

    #[test]
    fn human_gate_denies_without_token() {
        let agent = soc_agent();
        let verdict = agent
            .human_gate_verdict("isolate_host", "destructive", None)
            .unwrap();
        assert_eq!(verdict.decision, Decision::Deny);
        assert!(verdict.reason_codes.contains(&"AC-ASI09-gate".to_string()));
    }

    #[test]
    fn human_gate_allows_with_token() {
        use crate::config::DEFAULT_APPROVAL_TOKEN;

        let agent = soc_agent();
        let verdict = agent
            .human_gate_verdict("isolate_host", "destructive", Some(DEFAULT_APPROVAL_TOKEN))
            .unwrap();
        assert_eq!(verdict.decision, Decision::Allow);
        assert!(verdict
            .reason_codes
            .contains(&"AC-ASI09-gate-allow".to_string()));
    }
}
