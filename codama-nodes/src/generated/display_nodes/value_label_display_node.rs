use crate::{PluginNode, Text, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct ValueLabelDisplayNode {
    // Children.
    pub value: Box<ValueNode>,
    pub label: Text,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<ValueLabelDisplayNode> for crate::Node {
    fn from(val: ValueLabelDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
