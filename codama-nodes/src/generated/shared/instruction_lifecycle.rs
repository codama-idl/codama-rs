use serde::{Deserialize, Serialize};

/// The lifecycle stage of an instruction.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InstructionLifecycle {
    /// No longer included in client SDKs. Retained in the IDL for historical reference only.
    Archived,
    /// Still callable but discouraged. Clients should migrate to a replacement instruction.
    Deprecated,
    /// Work-in-progress. The instruction may change before it stabilises.
    Draft,
    /// Stable and supported for production use.
    Live,
}
