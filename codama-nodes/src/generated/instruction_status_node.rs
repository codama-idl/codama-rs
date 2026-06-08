use crate::InstructionLifecycle;
use codama_nodes_derive::node;

#[node]
pub struct InstructionStatusNode {
    // Data.
    pub lifecycle: InstructionLifecycle,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub message: Option<String>,
}
