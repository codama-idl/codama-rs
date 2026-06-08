use codama_nodes_derive::node;

#[node]
#[derive(Copy, Default)]
pub struct FixedCountNode {
    // Data.
    pub value: u64,
}

impl From<FixedCountNode> for crate::Node {
    fn from(val: FixedCountNode) -> Self {
        crate::Node::Count(val.into())
    }
}
