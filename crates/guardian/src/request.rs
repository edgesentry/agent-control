//! ACS-aligned hook request payloads for policy evaluation.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hook::Hook;

/// Minimal step context (ACS `StepContext` subset).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StepContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step_id: Option<String>,
}

/// Normalized hook invocation passed to the policy engine.
#[derive(Debug, Clone, PartialEq)]
pub struct HookRequest {
    pub hook: Hook,
    pub context: StepContext,
    /// Flattened text used for `content_contains` / regex rules.
    pub searchable_text: String,
    pub tool_id: Option<String>,
    pub trigger_type: Option<String>,
    /// Original ACS JSON-RPC `params` (for modify responses).
    pub params: Value,
}

impl HookRequest {
    pub fn from_jsonrpc(method: &str, params: Value) -> Result<Self, ParseError> {
        let hook = Hook::parse(method).ok_or(ParseError::UnknownMethod(method.to_string()))?;
        let context = parse_context(&params);
        let searchable_text = extract_searchable_text(hook, &params);
        let tool_id = params
            .pointer("/toolCallRequest/toolId")
            .or_else(|| params.pointer("/toolCallRequest/tool_id"))
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let trigger_type = params
            .pointer("/trigger/type")
            .and_then(|v| v.as_str())
            .map(str::to_string);

        Ok(Self {
            hook,
            context,
            searchable_text,
            tool_id,
            trigger_type,
            params,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unknown ACS method: {0}")]
    UnknownMethod(String),
}

fn parse_context(params: &Value) -> StepContext {
    let ctx = params.get("context").unwrap_or(params);
    StepContext {
        session_id: ctx
            .pointer("/session/id")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        agent_id: ctx
            .pointer("/agent/id")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        turn_id: ctx
            .pointer("/turnId")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        step_id: ctx
            .pointer("/stepId")
            .and_then(|v| v.as_str())
            .map(str::to_string),
    }
}

fn extract_searchable_text(hook: Hook, params: &Value) -> String {
    let mut parts = Vec::new();
    match hook {
        Hook::ToolCallRequest => {
            if let Some(id) = params
                .pointer("/toolCallRequest/toolId")
                .and_then(|v| v.as_str())
            {
                parts.push(id.to_string());
            }
            collect_input_values(params.pointer("/toolCallRequest/inputs"), &mut parts);
        }
        Hook::AgentTrigger => {
            if let Some(t) = params.pointer("/trigger/type").and_then(|v| v.as_str()) {
                parts.push(t.to_string());
            }
            collect_content_parts(params.pointer("/trigger/content"), &mut parts);
        }
        Hook::AgentResponse => {
            if let Some(role) = params.pointer("/message/role").and_then(|v| v.as_str()) {
                parts.push(role.to_string());
            }
            collect_content_parts(params.pointer("/message/content"), &mut parts);
        }
        Hook::KnowledgeRetrieval => {
            if let Some(q) = params
                .pointer("/knowledgeStep/query")
                .and_then(|v| v.as_str())
            {
                parts.push(q.to_string());
            }
            collect_content_parts(params.pointer("/knowledgeStep/results"), &mut parts);
        }
        Hook::MemoryStore => {
            if let Some(arr) = params.get("memory").and_then(|v| v.as_array()) {
                for item in arr {
                    if let Some(s) = item.as_str() {
                        parts.push(s.to_string());
                    }
                }
            }
        }
        Hook::Agbom => {
            push_string_field(params, "/artifact/id", &mut parts);
            push_string_field(params, "/artifact/name", &mut parts);
            push_string_field(params, "/registry/trust", &mut parts);
        }
        Hook::A2a => {
            push_string_field(params, "/delegate/agentId", &mut parts);
            push_string_field(params, "/message/body", &mut parts);
            collect_content_parts(params.pointer("/message/content"), &mut parts);
        }
        Hook::Trace => {
            push_string_field(params, "/correlation/status", &mut parts);
            push_string_field(params, "/correlation/pattern", &mut parts);
            if let Ok(s) = serde_json::to_string(&params) {
                parts.push(s);
            }
        }
        Hook::HumanGate => {
            push_string_field(params, "/approval/token", &mut parts);
            push_string_field(params, "/action/risk", &mut parts);
            push_string_field(params, "/action/toolId", &mut parts);
        }
    }
    parts.join("\n")
}

fn push_string_field(params: &Value, pointer: &str, parts: &mut Vec<String>) {
    if let Some(s) = params.pointer(pointer).and_then(|v| v.as_str()) {
        parts.push(s.to_string());
    }
}

fn collect_input_values(node: Option<&Value>, parts: &mut Vec<String>) {
    let Some(Value::Array(inputs)) = node else {
        return;
    };
    for input in inputs {
        if let Some(name) = input.get("name").and_then(|v| v.as_str()) {
            parts.push(name.to_string());
        }
        if let Some(value) = input.get("value").and_then(|v| v.as_str()) {
            parts.push(value.to_string());
        }
    }
}

fn collect_content_parts(node: Option<&Value>, parts: &mut Vec<String>) {
    let Some(Value::Array(items)) = node else {
        if let Some(s) = node.and_then(|v| v.as_str()) {
            parts.push(s.to_string());
        }
        return;
    };
    for item in items {
        if let Some(text) = item.pointer("/text").and_then(|v| v.as_str()) {
            parts.push(text.to_string());
        }
        if let Some(data) = item.get("data") {
            if let Ok(s) = serde_json::to_string(data) {
                parts.push(s);
            }
        }
    }
}
