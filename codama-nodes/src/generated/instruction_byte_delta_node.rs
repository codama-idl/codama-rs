use crate::InstructionByteDeltaValue;
use codama_nodes_derive::node;

#[node]
pub struct InstructionByteDeltaNode {
    // Data.
    pub with_header: bool,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub subtract: Option<bool>,

    // Children.
    pub value: Box<InstructionByteDeltaValue>,
}
