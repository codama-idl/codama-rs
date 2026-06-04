use crate::ValueNode;
use codama_nodes_derive::node;

#[node]
pub struct MapEntryValueNode {
    // Children.
    pub key: Box<ValueNode>,
    pub value: Box<ValueNode>,
}

impl From<MapEntryValueNode> for crate::Node {
    fn from(val: MapEntryValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
