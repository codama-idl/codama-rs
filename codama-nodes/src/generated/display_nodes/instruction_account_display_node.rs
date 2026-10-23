use crate::{DisplaySkip, PluginNode, Text};
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct InstructionAccountDisplayNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub skip: Option<DisplaySkip>,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub label: Option<Text>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<InstructionAccountDisplayNode> for crate::Node {
    fn from(val: InstructionAccountDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
