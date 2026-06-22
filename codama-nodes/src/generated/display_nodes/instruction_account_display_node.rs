use crate::DisplaySkip;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionAccountDisplayNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub skip: Option<DisplaySkip>,
}

impl From<InstructionAccountDisplayNode> for crate::Node {
    fn from(val: InstructionAccountDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
