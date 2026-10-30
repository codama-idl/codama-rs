use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ArgumentValueNode {
    // Data.
    pub name: CamelCaseString,
}

impl From<ArgumentValueNode> for crate::Node {
    fn from(val: ArgumentValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}
