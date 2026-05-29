//! Default analyst approval token for CS01 gate demo (override via CLI / env).
pub const DEFAULT_APPROVAL_TOKEN: &str = "soc-analyst-demo-token";

/// Destructive remediation tool used in gate demo (must match `policies/soc` tool_id_contains).
pub const DEMO_DESTRUCTIVE_TOOL: &str = "isolate_host";
