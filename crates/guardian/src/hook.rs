//! ACS Instrument hook identifiers (submission MVP subset).

/// Hook point aligned with ACS `steps/*` RPC methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Hook {
    AgentTrigger,
    ToolCallRequest,
    AgentResponse,
    KnowledgeRetrieval,
    MemoryStore,
}

impl Hook {
    /// ACS JSON-RPC `method` string for this hook.
    pub const fn acs_method(self) -> &'static str {
        match self {
            Self::AgentTrigger => "steps/agentTrigger",
            Self::ToolCallRequest => "steps/toolCallRequest",
            Self::AgentResponse => "steps/message",
            Self::KnowledgeRetrieval => "steps/knowledgeRetrieval",
            Self::MemoryStore => "steps/memoryStore",
        }
    }

    /// Parse hook from ACS method name or camelCase policy id.
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "steps/agentTrigger" | "agentTrigger" => Some(Self::AgentTrigger),
            "steps/toolCallRequest" | "toolCallRequest" => Some(Self::ToolCallRequest),
            "steps/message" | "agentResponse" | "message" => Some(Self::AgentResponse),
            "steps/knowledgeRetrieval" | "knowledgeRetrieval" => Some(Self::KnowledgeRetrieval),
            "steps/memoryStore" | "memoryStore" => Some(Self::MemoryStore),
            _ => None,
        }
    }
}
