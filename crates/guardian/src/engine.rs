//! Guardian policy engine entry point.

use std::path::Path;

use crate::decision::Verdict;
use crate::jsonrpc::{JsonRpcRequest, JsonRpcSuccessResponse};
use crate::policy::{PolicyError, PolicySet};
use crate::request::{HookRequest, ParseError};

/// ACS Instrument Guardian — synchronous policy evaluation.
#[derive(Debug, Clone)]
pub struct Guardian {
    policies: PolicySet,
}

impl Guardian {
    pub fn new(policies: PolicySet) -> Self {
        Self { policies }
    }

    pub fn load_from_dir(dir: &Path) -> Result<Self, PolicyError> {
        Ok(Self::new(PolicySet::load_dir(dir)?))
    }

    pub fn policies(&self) -> &PolicySet {
        &self.policies
    }

    /// Evaluate a normalized hook request before the observed action proceeds.
    pub fn evaluate(&self, request: &HookRequest) -> Verdict {
        self.policies.evaluate(request)
    }

    /// Evaluate an ACS JSON-RPC request and return an `ACSSuccessResponse`-shaped body.
    pub fn evaluate_jsonrpc(
        &self,
        request: &JsonRpcRequest,
    ) -> Result<JsonRpcSuccessResponse, EvaluateError> {
        let hook_request = request.to_hook_request()?;
        let verdict = self.evaluate(&hook_request);
        Ok(JsonRpcSuccessResponse::from_verdict(
            request.id.clone(),
            &request.method,
            verdict,
        ))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluateError {
    #[error(transparent)]
    Parse(#[from] ParseError),
}
