use crate::ConstantValueNode;
use codama_nodes_derive::node;

#[node]
pub struct ConstantDiscriminatorNode {
    // Data.
    pub offset: u64,

    // Children.
    pub constant: ConstantValueNode,
}

impl From<ConstantDiscriminatorNode> for crate::Node {
    fn from(val: ConstantDiscriminatorNode) -> Self {
        crate::Node::Discriminator(val.into())
    }
}
