use crate::{PluginNode, Text};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionDisplayNode {
    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub intent: Option<Text>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub interpolated_intent: Option<Text>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<InstructionDisplayNode> for crate::Node {
    fn from(val: InstructionDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
