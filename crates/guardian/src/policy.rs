//! Declarative YAML policy packs.

use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;

use crate::decision::{Decision, Verdict};
use crate::hook::Hook;
use crate::request::HookRequest;

/// Root document for a `.yaml` policy file.
#[derive(Debug, Clone, Deserialize)]
pub struct PolicyFile {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub rules: Vec<PolicyRule>,
}

fn default_version() -> String {
    "1".to_string()
}

/// Single rule: first match wins (file order, then load order).
#[derive(Debug, Clone, Deserialize)]
pub struct PolicyRule {
    pub id: String,
    pub hooks: Vec<String>,
    #[serde(default)]
    pub owasp: Vec<String>,
    #[serde(default)]
    pub r#match: MatchExpr,
    pub decision: Decision,
    pub message: String,
    #[serde(default)]
    pub reason_codes: Vec<String>,
    #[serde(default)]
    pub modify: Option<ModifySpec>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MatchExpr {
    #[serde(default)]
    pub tool_id_contains: Vec<String>,
    #[serde(default)]
    pub content_contains: Vec<String>,
    /// If any needle appears in searchable text, this rule does not match.
    #[serde(default)]
    pub unless_content_contains: Vec<String>,
    #[serde(default)]
    pub trigger_type_equals: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifySpec {
    /// Replace searchable payload with this string (agentResponse / memory).
    #[serde(default)]
    pub set_content: Option<String>,
    /// Prefix added to message text parts (agentResponse).
    #[serde(default)]
    pub content_prefix: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PolicySet {
    rules: Vec<PolicyRule>,
}

impl PolicySet {
    pub fn load_dir(dir: &Path) -> Result<Self, PolicyError> {
        if !dir.is_dir() {
            return Err(PolicyError::NotADirectory(dir.to_path_buf()));
        }
        let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension()
                    .and_then(|s| s.to_str())
                    .is_some_and(|ext| ext == "yaml" || ext == "yml")
            })
            .collect();
        paths.sort();
        let mut rules = Vec::new();
        for path in paths {
            rules.extend(PolicyFile::load_file(&path)?.rules);
        }
        Ok(Self { rules })
    }

    pub fn from_rules(rules: Vec<PolicyRule>) -> Self {
        Self { rules }
    }

    pub fn rules(&self) -> &[PolicyRule] {
        &self.rules
    }

    pub fn evaluate(&self, request: &HookRequest) -> Verdict {
        for rule in &self.rules {
            if let Some(verdict) = rule.apply(request) {
                return verdict;
            }
        }
        Verdict::allow("No matching policy; default allow")
    }
}

impl PolicyFile {
    pub fn load_file(path: &Path) -> Result<Self, PolicyError> {
        let raw = std::fs::read_to_string(path)?;
        let file: PolicyFile = serde_yaml::from_str(&raw)?;
        Ok(file)
    }
}

impl PolicyRule {
    fn applies_to_hook(&self, hook: Hook) -> bool {
        self.hooks
            .iter()
            .any(|name| Hook::parse(name) == Some(hook))
    }

    fn matches(&self, request: &HookRequest) -> bool {
        if !self.applies_to_hook(request.hook) {
            return false;
        }
        let m = &self.r#match;
        let text = request.searchable_text.to_lowercase();
        if m.unless_content_contains
            .iter()
            .any(|needle| text.contains(&needle.to_lowercase()))
        {
            return false;
        }
        if m.tool_id_contains.iter().any(|needle| {
            request
                .tool_id
                .as_deref()
                .is_some_and(|id| id.to_lowercase().contains(&needle.to_lowercase()))
                || text.contains(&needle.to_lowercase())
        }) {
            return true;
        }
        if m.content_contains
            .iter()
            .any(|needle| text.contains(&needle.to_lowercase()))
        {
            return true;
        }
        if let Some(ref expected) = m.trigger_type_equals {
            if request
                .trigger_type
                .as_deref()
                .is_some_and(|t| t.eq_ignore_ascii_case(expected))
            {
                return true;
            }
        }
        false
    }

    fn apply(&self, request: &HookRequest) -> Option<Verdict> {
        if !self.matches(request) {
            return None;
        }
        let mut verdict = match self.decision {
            Decision::Allow => Verdict::allow(&self.message),
            Decision::Deny => Verdict::deny(&self.message),
            Decision::Modify => self.build_modify(request)?,
        };
        verdict.matched_policy_ids = vec![self.id.clone()];
        verdict.reason_codes = self.reason_codes.clone();
        if !self.owasp.is_empty() {
            let owasp = format!("owasp={}", self.owasp.join(","));
            verdict.reasoning = Some(match verdict.reasoning {
                Some(r) => format!("{r}; {owasp}"),
                None => owasp,
            });
        }
        Some(verdict)
    }

    fn build_modify(&self, request: &HookRequest) -> Option<Verdict> {
        let spec = self.modify.as_ref()?;
        let mut params = request.params.clone();
        if let Some(prefix) = &spec.content_prefix {
            apply_message_prefix(&mut params, prefix);
        }
        if let Some(content) = &spec.set_content {
            set_message_text(&mut params, content);
        }
        Some(Verdict::modify(
            &self.message,
            params,
            vec![self.id.clone()],
        ))
    }
}

fn apply_message_prefix(params: &mut Value, prefix: &str) {
    let Some(content) = params.pointer_mut("/message/content") else {
        return;
    };
    let Some(parts) = content.as_array_mut() else {
        return;
    };
    for part in parts {
        if let Some(text) = part.get("text").and_then(|v| v.as_str()) {
            part["text"] = Value::String(format!("{prefix}{text}"));
        }
    }
}

fn set_message_text(params: &mut Value, text: &str) {
    if params.get("message").is_some() {
        params["message"]["content"] = serde_json::json!([{ "kind": "text", "text": text }]);
    } else if params.get("memory").is_some() {
        params["memory"] = serde_json::json!([text]);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PolicyError {
    #[error("not a directory: {}", .0.display())]
    NotADirectory(PathBuf),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
}
