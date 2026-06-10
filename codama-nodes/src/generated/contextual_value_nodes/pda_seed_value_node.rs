use crate::{CamelCaseString, HasName, PdaSeedValueValue};
use codama_nodes_derive::node;

#[node]
pub struct PdaSeedValueNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub value: Box<PdaSeedValueValue>,
}

impl From<PdaSeedValueNode> for crate::Node {
    fn from(val: PdaSeedValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl HasName for PdaSeedValueNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
