use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct SolAmountTypeNode {
    // Children.
    pub number: NestedTypeNode<NumberTypeNode>,
}

impl From<SolAmountTypeNode> for crate::Node {
    fn from(val: SolAmountTypeNode) -> Self {
        crate::Node::Type(val.into())
    }
}

impl SolAmountTypeNode {
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
        let node = SolAmountTypeNode::new(NumberTypeNode::new(U64, Endian::Big));
        assert_eq!(
            node.number,
            NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big))
        );
    }

    #[test]
    fn new_with_explicit_value() {
        let node =
            SolAmountTypeNode::new(NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big)));
        assert_eq!(
            node.number,
            NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big))
        );
    }

    #[test]
    fn new_with_nested_value() {
        let node = SolAmountTypeNode::new(PostOffsetTypeNode::pre_offset(
            PreOffsetTypeNode::absolute(NumberTypeNode::new(U64, Endian::Big), 0),
            0,
        ));
        assert_eq!(
            node.number,
            NestedTypeNode::PostOffset(Box::new(PostOffsetTypeNode::pre_offset(
                NestedTypeNode::PreOffset(Box::new(PreOffsetTypeNode::absolute(
                    NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big)),
                    0
                ))),
                0,
            )))
        );
        assert_eq!(
            node.number.get_nested_type_node(),
            &NumberTypeNode::new(U64, Endian::Big)
        );
    }

    #[test]
    fn to_json() {
        let node = SolAmountTypeNode::new(NumberTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"solAmountTypeNode","number":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"solAmountTypeNode","number":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#;
        let node: SolAmountTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, SolAmountTypeNode::new(NumberTypeNode::le(U64)));
    }
}
