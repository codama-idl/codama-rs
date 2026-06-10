use codama_nodes_derive::node;

#[node]
#[derive(Copy)]
pub struct BooleanValueNode {
    // Data.
    pub boolean: bool,
}

impl From<BooleanValueNode> for crate::Node {
    fn from(val: BooleanValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
