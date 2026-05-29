//! ACS JSON-RPC 2.0 request/response shapes (Instrument subset).

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::decision::Verdict;
use crate::request::{HookRequest, ParseError};

/// Incoming JSON-RPC 2.0 call from an Observed Agent.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub id: Value,
    #[serde(default)]
    pub params: Value,
}

/// Successful ACS Instrument response (`ACSSuccessResponse`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcSuccessResponse {
    pub jsonrpc: String,
    pub id: Value,
    pub result: AcsSuccessResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcsSuccessResult {
    pub decision: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_request: Option<ModifiedRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifiedRequest {
    pub method: String,
    pub params: Value,
}

impl JsonRpcSuccessResponse {
    pub fn from_verdict(id: Value, method: &str, verdict: Verdict) -> Self {
        let decision = match verdict.decision {
            crate::decision::Decision::Allow => "allow",
            crate::decision::Decision::Deny => "deny",
            crate::decision::Decision::Modify => "modify",
        };
        let modified_request = verdict.modified_params.map(|params| ModifiedRequest {
            method: method.to_string(),
            params,
        });
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: AcsSuccessResult {
                decision: decision.to_string(),
                message: verdict.message,
                reasoning: verdict.reasoning,
                reason_code: verdict.reason_codes,
                modified_request,
            },
        }
    }
}

impl JsonRpcRequest {
    pub fn to_hook_request(&self) -> Result<HookRequest, ParseError> {
        HookRequest::from_jsonrpc(&self.method, self.params.clone())
    }
}
