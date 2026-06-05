use crate::{CamelCaseString, HasName};
use codama_nodes_derive::node;

#[node]
pub struct ArgumentValueNode {
    // Data.
    pub name: CamelCaseString,
}

impl From<ArgumentValueNode> for crate::Node {
    fn from(val: ArgumentValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl HasName for ArgumentValueNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
