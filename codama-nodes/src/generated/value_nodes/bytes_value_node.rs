use crate::BytesEncoding;
use codama_nodes_derive::node;

#[node]
pub struct BytesValueNode {
    // Data.
    pub data: String,
    pub encoding: BytesEncoding,
}

impl From<BytesValueNode> for crate::Node {
    fn from(val: BytesValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
