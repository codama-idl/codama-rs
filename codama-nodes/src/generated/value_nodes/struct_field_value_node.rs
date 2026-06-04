use crate::{CamelCaseString, HasName, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct StructFieldValueNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub value: Box<ValueNode>,
}

impl From<StructFieldValueNode> for crate::Node {
    fn from(val: StructFieldValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}

impl HasName for StructFieldValueNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
