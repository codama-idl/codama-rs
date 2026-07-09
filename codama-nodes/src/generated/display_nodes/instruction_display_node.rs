use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionDisplayNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub intent: Option<String>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub interpolated_intent: Option<String>,
}

impl From<InstructionDisplayNode> for crate::Node {
    fn from(val: InstructionDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
