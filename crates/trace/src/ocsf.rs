//! Map ACS Guardian spans to OCSF Detection Finding JSON (v1.1 subset).

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::span::HookSpan;

/// OCSF Detection Finding — fields required for SIEM ingest (class 2004).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcsfEvent {
    pub activity_id: u32,
    pub activity_name: String,
    pub category_uid: u32,
    pub category_name: String,
    pub class_uid: u32,
    pub class_name: String,
    pub type_uid: u64,
    pub type_name: String,
    pub severity_id: u32,
    pub severity: String,
    pub time: i64,
    pub metadata: OcsfMetadata,
    pub finding_info: OcsfFindingInfo,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observables: Option<Vec<OcsfObservable>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unmapped: Option<Map<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcsfMetadata {
    pub version: String,
    pub uid: String,
    pub product: OcsfProduct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcsfProduct {
    pub name: String,
    pub vendor_name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcsfFindingInfo {
    pub uid: String,
    pub title: String,
    pub desc: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcsfObservable {
    pub name: String,
    pub r#type: String,
    pub value: String,
}

impl HookSpan {
    /// Convert span to OCSF Detection Finding (Create activity).
    pub fn to_ocsf(&self) -> OcsfEvent {
        let g = &self.guardian;
        let (severity_id, severity) = severity_for(&g.decision);
        let title = format!("Guardian {} on {}", g.decision, g.hook);
        let mut types = g.reason_codes.clone();
        if types.is_empty() {
            types.push(format!("acs.hook.{}", g.hook));
        }

        let mut unmapped = Map::new();
        unmapped.insert(
            "acs".into(),
            serde_json::json!({
                "method": g.acs_method,
                "decision": g.decision,
                "matched_policy_ids": g.matched_policy_ids,
                "probe_id": g.probe_id,
                "owasp_ids": g.owasp_ids,
                "session_id": g.session_id,
                "agent_id": g.agent_id,
            }),
        );
        unmapped.insert(
            "otel".into(),
            serde_json::json!({
                "trace_id": self.trace_id,
                "span_id": self.span_id,
                "span_name": self.name,
                "duration_ms": self.duration_ms,
            }),
        );

        let observables = build_observables(g);

        OcsfEvent {
            activity_id: 1,
            activity_name: "Create".into(),
            category_uid: 2,
            category_name: "Findings".into(),
            class_uid: 2004,
            class_name: "Detection Finding".into(),
            type_uid: 200401,
            type_name: "Detection Finding: Create".into(),
            severity_id,
            severity: severity.to_string(),
            time: self.start_time_ms,
            metadata: OcsfMetadata {
                version: "1.1.0".into(),
                uid: self.span_id.clone(),
                product: OcsfProduct {
                    name: "agent-control".into(),
                    vendor_name: "EdgeSentry".into(),
                    version: env!("CARGO_PKG_VERSION").into(),
                },
            },
            finding_info: OcsfFindingInfo {
                uid: format!("finding-{}", self.span_id),
                title,
                desc: g.message.clone(),
                types,
            },
            observables,
            unmapped: Some(unmapped),
        }
    }
}

fn severity_for(decision: &str) -> (u32, &'static str) {
    match decision {
        "deny" => (4, "High"),
        "modify" => (3, "Medium"),
        _ => (1, "Informational"),
    }
}

fn build_observables(g: &crate::span::GuardianRecord) -> Option<Vec<OcsfObservable>> {
    let mut obs = Vec::new();
    if let Some(ref id) = g.probe_id {
        obs.push(OcsfObservable {
            name: "probe_id".into(),
            r#type: "ACS Smoke Probe".into(),
            value: id.clone(),
        });
    }
    for owasp in &g.owasp_ids {
        obs.push(OcsfObservable {
            name: "owasp_id".into(),
            r#type: "OWASP Risk".into(),
            value: owasp.clone(),
        });
    }
    if obs.is_empty() {
        None
    } else {
        Some(obs)
    }
}

/// Serialize one OCSF event to pretty JSON.
pub fn to_json_pretty(event: &OcsfEvent) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(event)
}

/// Write OCSF JSON to a file (creates parent directories if needed).
pub fn write_json(path: &Path, event: &OcsfEvent) -> Result<(), OcsfError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, to_json_pretty(event)?)?;
    Ok(())
}

/// Write all spans in a trace batch as JSON array.
pub fn write_batch(path: &Path, events: &[OcsfEvent]) -> Result<(), OcsfError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, serde_json::to_string_pretty(events)?)?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum OcsfError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use crate::span::{GuardianRecord, Tracer};

    #[test]
    fn deny_tool_maps_to_detection_finding() {
        let mut tracer = Tracer::new();
        let span = tracer.record_guardian(
            "guardian.toolCallRequest",
            GuardianRecord {
                hook: "toolCallRequest".into(),
                acs_method: "steps/toolCallRequest".into(),
                decision: "deny".into(),
                message: "Dangerous tool invocation blocked by Guardian (ASI05)".into(),
                reason_codes: vec!["AC-ASI05-exec".into()],
                matched_policy_ids: vec!["p0-deny-shell-exec".into()],
                session_id: Some("lab-session".into()),
                agent_id: Some("lab-cicd-agent".into()),
                probe_id: Some("AC-ASI05-exec".into()),
                owasp_ids: vec!["ASI05:2026".into()],
            },
            2,
        );
        let ocsf = span.to_ocsf();
        assert_eq!(ocsf.class_uid, 2004);
        assert_eq!(ocsf.activity_id, 1);
        assert_eq!(ocsf.severity_id, 4);
        assert_eq!(ocsf.finding_info.types, vec!["AC-ASI05-exec"]);
        assert!(ocsf.unmapped.as_ref().unwrap().contains_key("acs"));
    }
}
