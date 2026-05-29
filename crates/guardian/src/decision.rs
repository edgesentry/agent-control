//! Guardian verdict (`allow` / `deny` / `modify`).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// ACS Guardian decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Decision {
    Allow,
    Deny,
    Modify,
}

/// Result returned before the observed action proceeds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Verdict {
    pub decision: Decision,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_codes: Vec<String>,
    /// OWASP / policy ids that matched (audit).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub matched_policy_ids: Vec<String>,
    /// When `decision == modify`, optional patched ACS `params` object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified_params: Option<Value>,
}

impl Verdict {
    pub fn allow(message: impl Into<String>) -> Self {
        Self {
            decision: Decision::Allow,
            message: message.into(),
            reasoning: None,
            reason_codes: vec![],
            matched_policy_ids: vec![],
            modified_params: None,
        }
    }

    pub fn deny(message: impl Into<String>) -> Self {
        Self {
            decision: Decision::Deny,
            message: message.into(),
            reasoning: None,
            reason_codes: vec![],
            matched_policy_ids: vec![],
            modified_params: None,
        }
    }

    pub fn modify(
        message: impl Into<String>,
        modified_params: Value,
        matched_policy_ids: Vec<String>,
    ) -> Self {
        Self {
            decision: Decision::Modify,
            message: message.into(),
            reasoning: None,
            reason_codes: vec![],
            matched_policy_ids,
            modified_params: Some(modified_params),
        }
    }

    pub fn with_reasoning(mut self, reasoning: impl Into<String>) -> Self {
        self.reasoning = Some(reasoning.into());
        self
    }

    pub fn with_reason_codes(mut self, codes: Vec<String>) -> Self {
        self.reason_codes = codes;
        self
    }
}
