//! OOTB alert-triage playbook — alert → enrich → recommend (no autonomous remediation).

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

use guardian::Decision;
use serde::{Deserialize, Serialize};
use trace::{write_batch, GuardianRecord, OcsfEvent, Tracer};

use crate::agent::SocAgent;

/// SIEM/SOAR alert fixture (mock sensor integration for submission MVP).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiemAlert {
    pub alert_id: String,
    pub source: String,
    pub rule: String,
    pub severity: String,
    pub timestamp: String,
    pub host: String,
    pub user: String,
    pub ioc: AlertIoc,
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertIoc {
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageReport {
    pub version: String,
    pub playbook: String,
    pub alert_id: String,
    pub severity: String,
    pub enrichment: EnrichmentSummary,
    pub recommendation: String,
    pub remediation_blocked: bool,
    pub blocked_tool: Option<String>,
    pub ocsf_event_count: u32,
    pub steps: Vec<TriageStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichmentSummary {
    pub ioc_type: String,
    pub ioc_value: String,
    pub reputation: String,
    pub related_alerts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageStep {
    pub step: String,
    pub hook: String,
    pub acs_method: String,
    pub decision: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_codes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ocsf_uid: Option<String>,
}

pub struct TriageOutput {
    pub report: TriageReport,
    pub ocsf_events: Vec<OcsfEvent>,
}

fn decision_label(d: Decision) -> &'static str {
    match d {
        Decision::Allow => "allow",
        Decision::Deny => "deny",
        Decision::Modify => "modify",
    }
}

struct StepMeta<'a> {
    step: &'a str,
    hook: &'a str,
    acs_method: &'a str,
    owasp_ids: &'a [&'a str],
}

fn record_step(
    tracer: &mut Tracer,
    agent: &SocAgent,
    meta: StepMeta<'_>,
    verdict: &guardian::Verdict,
    duration_ms: u64,
) -> (TriageStep, OcsfEvent) {
    let span = tracer.record_guardian(
        format!("guardian.{}", meta.hook),
        GuardianRecord {
            hook: meta.hook.to_string(),
            acs_method: meta.acs_method.to_string(),
            decision: decision_label(verdict.decision).to_string(),
            message: verdict.message.clone(),
            reason_codes: verdict.reason_codes.clone(),
            matched_policy_ids: verdict.matched_policy_ids.clone(),
            session_id: Some(agent.session_id().to_string()),
            agent_id: Some(agent.agent_id().to_string()),
            probe_id: Some(meta.step.to_string()),
            owasp_ids: meta.owasp_ids.iter().map(|s| (*s).to_string()).collect(),
        },
        duration_ms.max(1),
    );
    let ocsf = span.to_ocsf();
    let ocsf_uid = ocsf.metadata.uid.clone();
    let triage_step = TriageStep {
        step: meta.step.to_string(),
        hook: meta.hook.to_string(),
        acs_method: meta.acs_method.to_string(),
        decision: decision_label(verdict.decision).to_string(),
        message: verdict.message.clone(),
        reason_codes: verdict.reason_codes.clone(),
        ocsf_uid: Some(ocsf_uid),
    };
    (triage_step, ocsf)
}

pub fn load_alert(path: &Path) -> Result<SiemAlert, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse alert JSON: {e}"))
}

/// Run triage → enrich → recommend; attempt destructive remediation (expect deny).
pub fn run_triage(
    agent: &SocAgent,
    alert: &SiemAlert,
    report_path: Option<&Path>,
    trace_out: Option<&Path>,
) -> Result<TriageOutput, String> {
    let mut tracer = Tracer::new();
    let mut ocsf_events = Vec::new();
    let mut steps = Vec::new();

    // 1. agentTrigger — alert ingested
    let trigger_summary = format!(
        "alert_id={} severity={} rule={} host={} user={}",
        alert.alert_id, alert.severity, alert.rule, alert.host, alert.user
    );
    let started = Instant::now();
    let trigger_verdict = agent
        .on_alert_trigger(&trigger_summary)
        .map_err(|e| format!("agentTrigger: {e}"))?;
    let (step, ocsf) = record_step(
        &mut tracer,
        agent,
        StepMeta {
            step: "triage-start",
            hook: "agentTrigger",
            acs_method: "steps/agentTrigger",
            owasp_ids: &["ASI10:2026"],
        },
        &trigger_verdict,
        started.elapsed().as_millis() as u64,
    );
    steps.push(step);
    ocsf_events.push(ocsf);

    // 2. toolCallRequest — threat intel enrichment
    let started = Instant::now();
    let lookup_verdict = agent
        .tool_call_verdict(
            "lookup_threat_intel",
            &[
                ("ioc_type", &alert.ioc.r#type),
                ("ioc_value", &alert.ioc.value),
            ],
        )
        .map_err(|e| format!("lookup_threat_intel: {e}"))?;
    let (step, ocsf) = record_step(
        &mut tracer,
        agent,
        StepMeta {
            step: "enrich-ioc",
            hook: "toolCallRequest",
            acs_method: "steps/toolCallRequest",
            owasp_ids: &["LLM02:2025"],
        },
        &lookup_verdict,
        started.elapsed().as_millis() as u64,
    );
    steps.push(step);
    ocsf_events.push(ocsf);

    // 3. toolCallRequest — contextual enrichment
    let started = Instant::now();
    let enrich_verdict = agent
        .tool_call_verdict(
            "enrich_alert",
            &[("alert_id", &alert.alert_id), ("host", &alert.host)],
        )
        .map_err(|e| format!("enrich_alert: {e}"))?;
    let (step, ocsf) = record_step(
        &mut tracer,
        agent,
        StepMeta {
            step: "enrich-context",
            hook: "toolCallRequest",
            acs_method: "steps/toolCallRequest",
            owasp_ids: &["LLM02:2025"],
        },
        &enrich_verdict,
        started.elapsed().as_millis() as u64,
    );
    steps.push(step);
    ocsf_events.push(ocsf);

    // 4. toolCallRequest — destructive remediation attempt (must deny)
    let started = Instant::now();
    let isolate_verdict = agent
        .tool_call_verdict("isolate_host", &[("host", &alert.host)])
        .map_err(|e| format!("isolate_host: {e}"))?;
    let remediation_blocked = isolate_verdict.decision == Decision::Deny;
    let blocked_tool = Some("isolate_host".into());
    let (step, ocsf) = record_step(
        &mut tracer,
        agent,
        StepMeta {
            step: "remediation-attempt",
            hook: "toolCallRequest",
            acs_method: "steps/toolCallRequest",
            owasp_ids: &["ASI09:2026", "LLM06:2025"],
        },
        &isolate_verdict,
        started.elapsed().as_millis() as u64,
    );
    steps.push(step);
    ocsf_events.push(ocsf);

    if !remediation_blocked {
        return Err("expected isolate_host to be blocked by policies/soc".into());
    }

    // 5. agentResponse — analyst-facing recommendation (no autonomous action)
    let recommendation = format!(
        "Alert {} ({}) on host {}: IOC {}={} flagged as suspicious. \
         Recommend manual investigation and analyst-approved containment — \
         autonomous isolation blocked by Guardian.",
        alert.alert_id, alert.severity, alert.host, alert.ioc.r#type, alert.ioc.value
    );
    let started = Instant::now();
    let response_verdict = agent
        .respond(&recommendation)
        .map_err(|e| format!("agentResponse: {e}"))?;
    let (step, ocsf) = record_step(
        &mut tracer,
        agent,
        StepMeta {
            step: "recommend",
            hook: "agentResponse",
            acs_method: "steps/message",
            owasp_ids: &["ASI09:2026"],
        },
        &response_verdict,
        started.elapsed().as_millis() as u64,
    );
    steps.push(step);
    ocsf_events.push(ocsf);

    let enrichment = EnrichmentSummary {
        ioc_type: alert.ioc.r#type.clone(),
        ioc_value: alert.ioc.value.clone(),
        reputation: "suspicious".into(),
        related_alerts: 2,
    };

    let report = TriageReport {
        version: env!("CARGO_PKG_VERSION").into(),
        playbook: "alert-triage-enrich-recommend".into(),
        alert_id: alert.alert_id.clone(),
        severity: alert.severity.clone(),
        enrichment,
        recommendation,
        remediation_blocked,
        blocked_tool,
        ocsf_event_count: ocsf_events.len() as u32,
        steps,
    };

    if let Some(path) = report_path {
        let json =
            serde_json::to_string_pretty(&report).map_err(|e| format!("serialize report: {e}"))?;
        fs::write(path, json).map_err(|e| format!("write {}: {e}", path.display()))?;
    }

    if let Some(dir) = trace_out {
        write_batch(&dir.join("soc-ocsf-events.json"), &ocsf_events)
            .map_err(|e| format!("write OCSF batch: {e}"))?;
    }

    let _ = tracer.trace_id();

    Ok(TriageOutput {
        report,
        ocsf_events,
    })
}

pub fn print_triage_summary(report: &TriageReport, mut out: impl Write) -> io::Result<()> {
    writeln!(out, "SOC triage playbook: {}", report.playbook)?;
    writeln!(
        out,
        "Alert {} ({}) — {} OCSF events",
        report.alert_id, report.severity, report.ocsf_event_count
    )?;
    writeln!(
        out,
        "Enrichment: {}={} reputation={} related_alerts={}",
        report.enrichment.ioc_type,
        report.enrichment.ioc_value,
        report.enrichment.reputation,
        report.enrichment.related_alerts
    )?;
    if report.remediation_blocked {
        writeln!(
            out,
            "Remediation blocked: {} (Guardian deny)",
            report.blocked_tool.as_deref().unwrap_or("unknown")
        )?;
    }
    writeln!(out, "Recommendation: {}", report.recommendation)?;
    for step in &report.steps {
        writeln!(
            out,
            "  [{}] {} → {} ({})",
            step.step, step.hook, step.decision, step.message
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_alert() -> SiemAlert {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/alerts/sample-siem-alert.json");
        load_alert(&path).expect("sample alert")
    }

    fn triage_agent() -> SocAgent {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../policies/soc");
        let g = guardian::Guardian::load_from_dir(&dir).expect("policies/soc");
        SocAgent::new(g)
    }

    #[test]
    fn triage_playbook_five_ocsf_events() {
        let agent = triage_agent();
        let alert = fixture_alert();
        let out = run_triage(&agent, &alert, None, None).expect("triage");
        assert_eq!(out.report.steps.len(), 5);
        assert_eq!(out.ocsf_events.len(), 5);
        assert!(out.report.remediation_blocked);
        assert!(out.report.steps.iter().all(|s| s.ocsf_uid.is_some()));
    }
}
