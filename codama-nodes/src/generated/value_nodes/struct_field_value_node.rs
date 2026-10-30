use crate::{CamelCaseString, HasIdentifier, PluginNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct StructFieldValueNode {
    // Data.
    pub identifier: CamelCaseString,

    // Children.
    pub value: Box<ValueNode>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}

impl From<StructFieldValueNode> for crate::Node {
    fn from(val: StructFieldValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}

impl HasIdentifier for StructFieldValueNode {
    fn identifier(&self) -> &CamelCaseString {
        &self.identifier
    }
}
