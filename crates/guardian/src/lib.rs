//! ACS Instrument: inline hooks and declarative policy evaluation.
//!
//! Maps ACS JSON-RPC `steps/*` methods to YAML policies and returns
//! `allow` / `deny` / `modify` verdicts before the observed action proceeds.

mod decision;
mod engine;
mod hook;
mod jsonrpc;
mod policy;
mod request;

pub use decision::{Decision, Verdict};
pub use engine::{EvaluateError, Guardian};
pub use hook::Hook;
pub use jsonrpc::{AcsSuccessResult, JsonRpcRequest, JsonRpcSuccessResponse, ModifiedRequest};
pub use policy::{MatchExpr, PolicyError, PolicyFile, PolicyRule, PolicySet};
pub use request::{HookRequest, ParseError, StepContext};

/// Crate version (matches workspace `0.1.0` until release tagging in issue #15).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests;
