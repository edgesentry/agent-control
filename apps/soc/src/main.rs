//! CS01 Governed SOC Agent — OOTB alert-triage Observed Agent.

mod agent;
mod triage;

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use agent::SocAgent;
use clap::{Parser, Subcommand};
use guardian::Guardian;
use triage::{load_alert, print_triage_summary, run_triage};

#[derive(Parser)]
#[command(
    name = "soc",
    about = "CS01 alert-triage Observed Agent (OOTB playbook)"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run alert-triage → enrich → recommend playbook on a SIEM fixture.
    Triage {
        /// Path to SIEM alert JSON fixture.
        #[arg(long, default_value = "examples/alerts/sample-siem-alert.json")]
        alert: PathBuf,
        /// Write JSON triage report to this path.
        #[arg(long)]
        report: Option<PathBuf>,
        /// Write OCSF JSON batch (env: `AGENT_CONTROL_TRACE_OUT`).
        #[arg(long, env = "AGENT_CONTROL_TRACE_OUT")]
        trace_out: Option<PathBuf>,
    },
}

fn policy_dir() -> PathBuf {
    env::var("AGENT_CONTROL_POLICY_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("policies/soc"))
}

fn load_guardian() -> Result<Guardian, ExitCode> {
    let dir = policy_dir();
    Guardian::load_from_dir(&dir).map_err(|e| {
        eprintln!("Failed to load policies from {}: {e}", dir.display());
        ExitCode::from(2)
    })
}

fn run_triage_cmd(
    g: Guardian,
    alert_path: PathBuf,
    report_path: Option<PathBuf>,
    trace_out: Option<PathBuf>,
) -> ExitCode {
    let alert = match load_alert(&alert_path) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };

    println!(
        "agent-control soc v{} (guardian {}, trace {})",
        env!("CARGO_PKG_VERSION"),
        guardian::VERSION,
        trace::VERSION,
    );
    println!(
        "Loaded {} policy rule(s) from {}",
        g.policies().rules().len(),
        policy_dir().display()
    );
    println!("Alert fixture: {}", alert_path.display());

    let agent = SocAgent::new(g);
    let out_path = report_path.clone();
    let trace_path = trace_out.clone();

    match run_triage(&agent, &alert, report_path.as_deref(), trace_out.as_deref()) {
        Ok(output) => {
            if let Err(e) = print_triage_summary(&output.report, std::io::stdout()) {
                eprintln!("Output error: {e}");
                return ExitCode::from(1);
            }
            if let Some(path) = out_path {
                println!("Report written to {}", path.display());
            }
            if let Some(dir) = trace_path {
                println!(
                    "OCSF export ({} events): {}/soc-ocsf-events.json",
                    output.ocsf_events.len(),
                    dir.display()
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Triage failed: {e}");
            ExitCode::from(1)
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let g = match load_guardian() {
        Ok(g) => g,
        Err(code) => return code,
    };

    match cli.command.unwrap_or(Command::Triage {
        alert: PathBuf::from("examples/alerts/sample-siem-alert.json"),
        report: None,
        trace_out: None,
    }) {
        Command::Triage {
            alert,
            report,
            trace_out,
        } => run_triage_cmd(g, alert, report, trace_out),
    }
}
