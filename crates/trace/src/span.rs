//! OTel-style span for an ACS Instrument hook evaluation.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// Guardian verdict + hook context captured on a span.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GuardianRecord {
    pub hook: String,
    pub acs_method: String,
    pub decision: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_codes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub matched_policy_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owasp_ids: Vec<String>,
}

/// Single span emitted around a Guardian hook evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookSpan {
    pub trace_id: String,
    pub span_id: String,
    pub name: String,
    pub start_time_ms: i64,
    pub duration_ms: u64,
    pub guardian: GuardianRecord,
}

static SPAN_COUNTER: AtomicU64 = AtomicU64::new(1);

fn epoch_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn next_id(prefix: &str) -> String {
    let n = SPAN_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}{n:012x}")
}

/// In-process tracer — lightweight OTel stand-in for submission MVP.
#[derive(Debug, Clone, Default)]
pub struct Tracer {
    trace_id: Option<String>,
    spans: Vec<HookSpan>,
}

impl Tracer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trace_id(&self) -> &str {
        self.trace_id
            .as_deref()
            .expect("tracer trace_id set after first span")
    }

    pub fn spans(&self) -> &[HookSpan] {
        &self.spans
    }

    /// Record a Guardian hook evaluation as a span (async export happens via OCSF mapper).
    pub fn record_guardian(
        &mut self,
        name: impl Into<String>,
        record: GuardianRecord,
        duration_ms: u64,
    ) -> &HookSpan {
        if self.trace_id.is_none() {
            self.trace_id = Some(next_id("trace-"));
        }
        let span = HookSpan {
            trace_id: self.trace_id.clone().unwrap(),
            span_id: next_id("span-"),
            name: name.into(),
            start_time_ms: epoch_ms().saturating_sub(duration_ms as i64),
            duration_ms,
            guardian: record,
        };
        tracing::info!(
            trace_id = %span.trace_id,
            span_id = %span.span_id,
            hook = %span.guardian.hook,
            decision = %span.guardian.decision,
            "guardian hook evaluated"
        );
        self.spans.push(span);
        self.spans.last().expect("just pushed")
    }
}
