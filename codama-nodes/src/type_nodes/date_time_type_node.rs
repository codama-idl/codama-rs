use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct DateTimeTypeNode {
    // Children.
    pub number: NestedTypeNode<NumberTypeNode>,
}

impl From<DateTimeTypeNode> for crate::Node {
    fn from(val: DateTimeTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl DateTimeTypeNode {
    pub fn new<T>(number: T) -> Self
    where
        T: Into<NestedTypeNode<NumberTypeNode>>,
    {
        Self {
            number: number.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Endian, NestedTypeNodeTrait, NumberTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, U64,
    };

    #[test]
    fn new() {
        let node = DateTimeTypeNode::new(NumberTypeNode::new(U64, Endian::Big));
        assert_eq!(
            node.number,
            NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big))
        );
    }

    #[test]
    fn new_with_nested_value() {
        let node = DateTimeTypeNode::new(PostOffsetTypeNode::pre_offset(
            PreOffsetTypeNode::absolute(NumberTypeNode::le(U64), 0),
            0,
        ));
        assert_eq!(
            node.number,
            NestedTypeNode::PostOffset(Box::new(PostOffsetTypeNode::pre_offset(
                NestedTypeNode::PreOffset(Box::new(PreOffsetTypeNode::absolute(
                    NestedTypeNode::Value(NumberTypeNode::le(U64)),
                    0
                ))),
                0,
            )))
        );
        assert_eq!(
            node.number.get_nested_type_node(),
            &NumberTypeNode::le(U64,)
        );
    }

    #[test]
    fn to_json() {
        let node = DateTimeTypeNode::new(NumberTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"dateTimeTypeNode","number":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"dateTimeTypeNode","number":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#;
        let node: DateTimeTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, DateTimeTypeNode::new(NumberTypeNode::le(U64)));
    }
}
