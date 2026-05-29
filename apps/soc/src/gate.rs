//! Analyst approval gate demo — deny without token, allow with token.

use std::io::{self, Write};

use guardian::Decision;
use serde::{Deserialize, Serialize};

use crate::agent::SocAgent;
use crate::config::DEMO_DESTRUCTIVE_TOOL;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateStep {
    pub phase: String,
    pub hook: String,
    pub decision: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateReport {
    pub version: String,
    pub destructive_tool: String,
    pub approval_token: String,
    pub passed: bool,
    pub steps: Vec<GateStep>,
}

fn decision_label(d: Decision) -> &'static str {
    match d {
        Decision::Allow => "allow",
        Decision::Deny => "deny",
        Decision::Modify => "modify",
    }
}

fn record(phase: &str, hook: &str, verdict: &guardian::Verdict) -> GateStep {
    GateStep {
        phase: phase.to_string(),
        hook: hook.to_string(),
        decision: decision_label(verdict.decision).to_string(),
        message: verdict.message.clone(),
        reason_codes: verdict.reason_codes.clone(),
    }
}

/// Run deny-without-token → allow-with-token gate demo (CS01 / ASI09).
pub fn run_gate_demo(
    agent: &SocAgent,
    host: &str,
    approval_token: &str,
) -> Result<GateReport, String> {
    let tool = DEMO_DESTRUCTIVE_TOOL;
    let mut steps = Vec::new();

    let deny_tool = agent
        .tool_call_verdict(tool, &[("host", host)])
        .map_err(|e| format!("toolCallRequest deny: {e}"))?;
    steps.push(record(
        "remediation-without-token",
        "toolCallRequest",
        &deny_tool,
    ));
    if deny_tool.decision != Decision::Deny {
        return Err("expected destructive toolCallRequest deny without token".into());
    }

    let deny_gate = agent
        .human_gate_verdict(tool, "destructive", None)
        .map_err(|e| format!("humanGate deny: {e}"))?;
    steps.push(record("humanGate-without-token", "humanGate", &deny_gate));
    if deny_gate.decision != Decision::Deny {
        return Err("expected humanGate deny without token".into());
    }

    let allow_gate = agent
        .human_gate_verdict(tool, "destructive", Some(approval_token))
        .map_err(|e| format!("humanGate allow: {e}"))?;
    steps.push(record("humanGate-with-token", "humanGate", &allow_gate));
    if allow_gate.decision != Decision::Allow {
        return Err("expected humanGate allow with token".into());
    }

    let allow_tool = agent
        .tool_call_verdict_with_approval(tool, &[("host", host)], Some(approval_token))
        .map_err(|e| format!("toolCallRequest allow: {e}"))?;
    steps.push(record(
        "remediation-with-token",
        "toolCallRequest",
        &allow_tool,
    ));
    if allow_tool.decision != Decision::Allow {
        return Err("expected destructive toolCallRequest allow with token".into());
    }

    Ok(GateReport {
        version: env!("CARGO_PKG_VERSION").into(),
        destructive_tool: tool.to_string(),
        approval_token: approval_token.to_string(),
        passed: true,
        steps,
    })
}

pub fn print_gate_summary(report: &GateReport, mut out: impl Write) -> io::Result<()> {
    writeln!(
        out,
        "Analyst gate demo: {} — {}",
        report.destructive_tool,
        if report.passed { "PASSED" } else { "FAILED" }
    )?;
    for step in &report.steps {
        writeln!(
            out,
            "  [{}] {} → {} ({})",
            step.phase, step.hook, step.decision, step.message
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn gate_agent() -> SocAgent {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../policies/soc");
        let g = guardian::Guardian::load_from_dir(&dir).expect("policies/soc");
        SocAgent::new(g)
    }

    #[test]
    fn gate_demo_four_steps_pass() {
        use crate::config::DEFAULT_APPROVAL_TOKEN;

        let agent = gate_agent();
        let report = run_gate_demo(&agent, "ws-finance-17", DEFAULT_APPROVAL_TOKEN).expect("gate");
        assert!(report.passed);
        assert_eq!(report.steps.len(), 4);
    }
}
