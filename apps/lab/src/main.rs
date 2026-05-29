//! CS02 Guardian Lab — CI/CD Observed Agent and P0 smoke CLI.

mod agent;
mod smoke;

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use agent::{LabAgent, ToolOutcome};
use clap::{Parser, Subcommand};
use guardian::Guardian;
use smoke::{print_smoke_summary, run_smoke};

#[derive(Parser)]
#[command(name = "lab", about = "CS02 CI/CD Observed Agent and P0 smoke harness")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run CI/CD agent demo (Guardian intercepts tool calls).
    Demo,
    /// Run P0 OWASP smoke probes through Guardian (`cargo run -p lab -- smoke`).
    Smoke {
        /// Write JSON report to this path (default: stdout only).
        #[arg(long)]
        report: Option<PathBuf>,
    },
}

fn policy_dir() -> PathBuf {
    env::var("AGENT_CONTROL_POLICY_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("policies/p0"))
}

fn load_guardian() -> Result<Guardian, ExitCode> {
    let dir = policy_dir();
    Guardian::load_from_dir(&dir).map_err(|e| {
        eprintln!("Failed to load policies from {}: {e}", dir.display());
        ExitCode::from(2)
    })
}

fn run_demo(g: Guardian) -> ExitCode {
    println!(
        "agent-control lab v{} (guardian {})",
        env!("CARGO_PKG_VERSION"),
        guardian::VERSION,
    );
    println!(
        "Loaded {} policy rule(s) from {}",
        g.policies().rules().len(),
        policy_dir().display()
    );

    let agent = LabAgent::new(g);

    match agent.invoke_tool("shell_exec", &[("cmd", "rm -rf /")]) {
        Ok(ToolOutcome::Blocked {
            tool_id, message, ..
        }) => {
            println!("Blocked dangerous tool `{tool_id}`: {message}");
        }
        Ok(ToolOutcome::Executed { tool_id }) => {
            eprintln!("Unexpected allow for `{tool_id}`");
            return ExitCode::from(1);
        }
        Err(e) => {
            eprintln!("Demo error: {e}");
            return ExitCode::from(1);
        }
    }

    match agent.invoke_tool("read_file", &[("path", "README.md")]) {
        Ok(ToolOutcome::Executed { tool_id }) => {
            println!("Allowed benign tool `{tool_id}`");
        }
        Ok(ToolOutcome::Blocked { tool_id, .. }) => {
            eprintln!("Unexpected block for `{tool_id}`");
            return ExitCode::from(1);
        }
        Err(e) => {
            eprintln!("Demo error: {e}");
            return ExitCode::from(1);
        }
    }

    ExitCode::SUCCESS
}

fn run_smoke_cmd(g: Guardian, report_path: Option<PathBuf>) -> ExitCode {
    let agent = LabAgent::new(g);
    let out_path = report_path.clone();
    match run_smoke(&agent, report_path.as_deref()) {
        Ok(report) => {
            if let Err(e) = print_smoke_summary(&report, std::io::stdout()) {
                eprintln!("Output error: {e}");
                return ExitCode::from(1);
            }
            if let Some(path) = out_path {
                println!("Report written to {}", path.display());
            }
            if report.failed > 0 {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            eprintln!("Smoke run failed: {e}");
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

    match cli.command.unwrap_or(Command::Demo) {
        Command::Demo => run_demo(g),
        Command::Smoke { report } => run_smoke_cmd(g, report),
    }
}
