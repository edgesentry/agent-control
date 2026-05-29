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
    /// ACS `inspect/agbom` (P0 smoke ASI04).
    Agbom,
    /// Agent-to-agent messaging (P0 smoke ASI07).
    A2a,
    /// Trace correlation export (P0 smoke ASI08).
    Trace,
    /// Analyst approval gate (P0 smoke ASI09).
    HumanGate,
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
            Self::Agbom => "inspect/agbom",
            Self::A2a => "a2a/send",
            Self::Trace => "trace/correlation",
            Self::HumanGate => "instrument/humanGate",
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
            "inspect/agbom" | "agbom" => Some(Self::Agbom),
            "a2a/send" | "a2a/message/stream" | "a2a" => Some(Self::A2a),
            "trace/correlation" | "trace" => Some(Self::Trace),
            "instrument/humanGate" | "humanGate" => Some(Self::HumanGate),
            _ => None,
        }
    }
}
