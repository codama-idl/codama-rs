use codama_nodes_derive::node;

#[node]
#[derive(Copy)]
pub struct SizeDiscriminatorNode {
    // Data.
    pub size: u64,
}

impl From<SizeDiscriminatorNode> for crate::Node {
    fn from(val: SizeDiscriminatorNode) -> Self {
        crate::Node::Discriminator(val.into())
    }
}
