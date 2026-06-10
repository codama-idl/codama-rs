use crate::MapEntryValueNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct MapValueNode {
    // Children.
    pub entries: Vec<MapEntryValueNode>,
}

impl From<MapValueNode> for crate::Node {
    fn from(val: MapValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}
