//! ACS Trace: OpenTelemetry → OCSF mapping for SIEM export.
//!
//! Full implementation: issue #5 (`crates/trace`).

/// Crate version (matches workspace `0.1.0` until release tagging in issue #15).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!VERSION.is_empty());
    }
}
