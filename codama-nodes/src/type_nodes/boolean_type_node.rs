use crate::{NestedTypeNode, NumberTypeNode, U8};
use codama_nodes_derive::type_node;

#[type_node]
pub struct BooleanTypeNode {
    // Children.
    pub size: NestedTypeNode<NumberTypeNode>,
}

impl From<BooleanTypeNode> for crate::Node {
    fn from(val: BooleanTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl BooleanTypeNode {
    pub fn new<T>(size: T) -> Self
    where
        T: Into<NestedTypeNode<NumberTypeNode>>,
    {
        Self { size: size.into() }
    }
}

impl Default for BooleanTypeNode {
    fn default() -> Self {
        Self::new(NumberTypeNode::le(U8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NestedTypeNodeTrait, NumberTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, U64};

    #[test]
    fn new() {
        let node = BooleanTypeNode::new(NumberTypeNode::le(U64));
        assert_eq!(node.size, NestedTypeNode::Value(NumberTypeNode::le(U64,)));
    }

    #[test]
    fn new_with_nested_size() {
        let node = BooleanTypeNode::new(PostOffsetTypeNode::pre_offset(
            PreOffsetTypeNode::absolute(NumberTypeNode::le(U64), 0),
            0,
        ));
        assert_eq!(
            node.size,
            NestedTypeNode::PostOffset(Box::new(PostOffsetTypeNode::pre_offset(
                NestedTypeNode::PreOffset(Box::new(PreOffsetTypeNode::absolute(
                    NestedTypeNode::Value(NumberTypeNode::le(U64)),
                    0
                ))),
                0,
            )))
        );
        assert_eq!(node.size.get_nested_type_node(), &NumberTypeNode::le(U64));
    }

    #[test]
    fn default() {
        let node = BooleanTypeNode::default();
        assert_eq!(node.size, NestedTypeNode::Value(NumberTypeNode::le(U8)));
    }

    #[test]
    fn to_json() {
        let node = BooleanTypeNode::new(NumberTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"booleanTypeNode","size":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"booleanTypeNode","size":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#;
        let node: BooleanTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, BooleanTypeNode::new(NumberTypeNode::le(U64)));
    }
}
