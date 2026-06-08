use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct StringValueNode {
    // Data.
    pub string: String,
}

impl From<StringValueNode> for crate::Node {
    fn from(val: StringValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
