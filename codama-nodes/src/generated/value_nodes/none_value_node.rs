use codama_nodes_derive::node;

#[node]
#[derive(Copy, Default)]
pub struct NoneValueNode {}

impl From<NoneValueNode> for crate::Node {
    fn from(val: NoneValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
