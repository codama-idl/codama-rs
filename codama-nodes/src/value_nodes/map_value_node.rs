use crate::MapEntryValueNode;
use codama_nodes_derive::node;

#[node]
pub struct MapValueNode {
    // Children.
    pub entries: Vec<MapEntryValueNode>,
}

impl From<MapValueNode> for crate::Node {
    fn from(val: MapValueNode) -> Self {
        crate::Node::Value(val.into())
    }
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

    #[test]
    fn to_json() {
        let node = MapValueNode::new(vec![MapEntryValueNode::new(
            StringValueNode::new("answer"),
            NumberValueNode::new(42),
        )]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"mapValueNode","entries":[{"kind":"mapEntryValueNode","key":{"kind":"stringValueNode","string":"answer"},"value":{"kind":"numberValueNode","number":42}}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"mapValueNode","entries":[{"kind":"mapEntryValueNode","key":{"kind":"stringValueNode","string":"answer"},"value":{"kind":"numberValueNode","number":42}}]}"#;
        let node: MapValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            MapValueNode::new(vec![MapEntryValueNode::new(
                StringValueNode::new("answer"),
                NumberValueNode::new(42u32),
            )])
        );
    }
}
