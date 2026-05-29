//! ACS Instrument: inline hooks and declarative policy evaluation.
//!
//! Full implementation: issue #4 (`crates/guardian`).

/// Crate version (matches workspace `0.1.0` until release tagging in issue #15).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// ACS Instrument hook identifiers planned for the submission MVP subset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hook {
    AgentTrigger,
    ToolCallRequest,
    AgentResponse,
    KnowledgeRetrieval,
    MemoryStore,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!VERSION.is_empty());
    }
}
