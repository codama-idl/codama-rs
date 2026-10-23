use crate::{CamelCaseString, HasIdentifier, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct StructFieldValueNode {
    // Data.
    // TODO(spec v2): keeps the v1 JSON key "name" temporarily so v1 output stays the same. The next v2 regeneration drops it.
    #[serde(rename = "name")]
    pub identifier: CamelCaseString,

    // Children.
    pub value: Box<ValueNode>,
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
