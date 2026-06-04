use codama_nodes_derive::node;

#[node]
#[derive(Copy, Default)]
pub struct RemainderCountNode {}

impl From<RemainderCountNode> for crate::Node {
    fn from(val: RemainderCountNode) -> Self {
        crate::Node::Count(val.into())
    }
}
