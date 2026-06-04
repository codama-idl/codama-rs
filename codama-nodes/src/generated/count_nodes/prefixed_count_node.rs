use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::node;

#[node]
pub struct PrefixedCountNode {
    // Children.
    pub prefix: NestedTypeNode<NumberTypeNode>,
}

impl From<PrefixedCountNode> for crate::Node {
    fn from(val: PrefixedCountNode) -> Self {
        crate::Node::Count(val.into())
    }
}
