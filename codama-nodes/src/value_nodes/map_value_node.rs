use crate::MapEntryValueNode;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct MapValueNode {
    // Children.
    pub entries: Vec<MapEntryValueNode>,
}

impl MapValueNode {
    pub fn new(entries: Vec<MapEntryValueNode>) -> Self {
        Self { entries }
    }
}

impl From<Vec<MapEntryValueNode>> for MapValueNode {
    fn from(items: Vec<MapEntryValueNode>) -> Self {
        Self::new(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberValueNode, StringValueNode, ValueNode};

    #[test]
    fn new() {
        let node = MapValueNode::new(vec![
            MapEntryValueNode::new(StringValueNode::new("Alice"), NumberValueNode::new(180)),
            MapEntryValueNode::new(StringValueNode::new("Bob"), NumberValueNode::new(125)),
        ]);
        assert_eq!(
            node.entries,
            vec![
                MapEntryValueNode::new(
                    ValueNode::String(StringValueNode::new("Alice")),
                    ValueNode::Number(NumberValueNode::new(180))
                ),
                MapEntryValueNode::new(
                    ValueNode::String(StringValueNode::new("Bob")),
                    ValueNode::Number(NumberValueNode::new(125))
                ),
            ]
        );
    }

    #[test]
    fn from_vec() {
        let node: MapValueNode = vec![MapEntryValueNode::new(
            StringValueNode::new("answer"),
            NumberValueNode::new(42),
        )]
        .into();
        assert_eq!(
            node.entries,
            vec![MapEntryValueNode::new(
                StringValueNode::new("answer"),
                NumberValueNode::new(42),
            )]
        );
    }
}
