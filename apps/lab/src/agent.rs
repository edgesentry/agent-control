//! CS02 CI/CD Observed Agent — routes agent steps through Guardian before execution.

use guardian::{Guardian, HookRequest, JsonRpcRequest};
use serde_json::{json, Value};
use thiserror::Error;

/// Minimal lab agent that simulates a coding assistant invoking CI/CD tools.
pub struct LabAgent {
    guardian: Guardian,
    session_id: String,
    agent_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolOutcome {
    /// Guardian allowed; tool would proceed (mock success).
    Executed { tool_id: String },
    /// Guardian denied the invocation.
    Blocked {
        tool_id: String,
        message: String,
        reason_codes: Vec<String>,
    },
}

#[derive(Debug, Error)]
pub enum LabError {
    #[error("guardian evaluation failed: {0}")]
    Evaluate(String),
}

impl LabAgent {
    pub fn new(guardian: Guardian) -> Self {
        Self {
            guardian,
            session_id: "lab-session".into(),
            agent_id: "lab-cicd-agent".into(),
        }
    }

    /// Intercept `toolCallRequest` via Guardian (ACS Instrument hook).
    pub fn invoke_tool(
        &self,
        tool_id: &str,
        inputs: &[(&str, &str)],
    ) -> Result<ToolOutcome, LabError> {
        let input_values: Vec<Value> = inputs
            .iter()
            .map(|(name, value)| {
                json!({
                    "name": name,
                    "value": value
                })
            })
            .collect();

        let rpc = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "steps/toolCallRequest".into(),
            id: json!(format!("lab-{tool_id}")),
            params: json!({
                "toolCallRequest": {
                    "executionId": format!("exec-{tool_id}"),
                    "toolId": tool_id,
                    "inputs": input_values
                },
                "context": {
                    "session": { "id": self.session_id },
                    "agent": { "id": self.agent_id, "name": "lab" }
                }
            }),
        };

        let resp = self
            .guardian
            .evaluate_jsonrpc(&rpc)
            .map_err(|e| LabError::Evaluate(e.to_string()))?;

        match resp.result.decision.as_str() {
            "deny" => Ok(ToolOutcome::Blocked {
                tool_id: tool_id.to_string(),
                message: resp.result.message,
                reason_codes: resp.result.reason_code,
            }),
            "allow" | "modify" => Ok(ToolOutcome::Executed {
                tool_id: tool_id.to_string(),
            }),
            other => Err(LabError::Evaluate(format!("unexpected decision: {other}"))),
        }
    }

    /// Evaluate any ACS JSON-RPC step through Guardian (smoke harness).
    pub fn evaluate_step(
        &self,
        method: &str,
        params: Value,
        _request_id: &str,
    ) -> Result<guardian::Verdict, LabError> {
        let hook = HookRequest::from_jsonrpc(method, params)
            .map_err(|e| LabError::Evaluate(e.to_string()))?;
        Ok(self.guardian.evaluate(&hook))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn lab_agent() -> LabAgent {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../policies/p0");
        let g = Guardian::load_from_dir(&dir).expect("policies/p0");
        LabAgent::new(g)
    }

    #[test]
    fn blocks_shell_tool() {
        let agent = lab_agent();
        let out = agent.invoke_tool("bash_shell", &[]).unwrap();
        assert!(matches!(out, ToolOutcome::Blocked { ref tool_id, .. } if tool_id == "bash_shell"));
    }

    #[test]
    fn allows_benign_read() {
        let agent = lab_agent();
        let out = agent
            .invoke_tool("read_file", &[("path", "README.md")])
            .unwrap();
        assert!(matches!(
            out,
            ToolOutcome::Executed { ref tool_id } if tool_id == "read_file"
        ));
    }
}
