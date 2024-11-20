use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::{node, TypeNode};

#[node]
#[derive(TypeNode)]
pub struct AmountTypeNode {
    // Data.
    pub decimals: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    // Children.
    pub number: NestedTypeNode<NumberTypeNode>,
}

impl AmountTypeNode {
    pub fn new<T>(number: T, decimals: u8, unit: Option<String>) -> Self
    where
        T: Into<NestedTypeNode<NumberTypeNode>>,
    {
        Self {
            decimals,
            unit,
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
        let node = AmountTypeNode::new(NumberTypeNode::new(U64, Endian::Big), 0, None);
        assert_eq!(
            node.number,
            NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big))
        );
        assert_eq!(node.decimals, 0);
        assert_eq!(node.unit, None);
    }

    #[test]
    fn new_with_offset() {
        let node = AmountTypeNode::new(
            NumberTypeNode::new(U64, Endian::Big),
            9,
            Some("SOL".to_string()),
        );
        assert_eq!(
            node.number,
            NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big))
        );
        assert_eq!(node.decimals, 9);
        assert_eq!(node.unit, Some("SOL".to_string()));
    }

    #[test]
    fn new_with_explicit_value() {
        let node = AmountTypeNode::new(
            NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big)),
            0,
            None,
        );
        assert_eq!(
            node.number,
            NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big))
        );
    }

    #[test]
    fn new_with_nested_value() {
        let node = AmountTypeNode::new(
            PostOffsetTypeNode::pre_offset(
                PreOffsetTypeNode::absolute(NumberTypeNode::new(U64, Endian::Big), 0),
                0,
            ),
            9,
            Some("SOL".to_string()),
        );
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
        let node = AmountTypeNode::new(NumberTypeNode::new(U64, Endian::Little), 9, None);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"amountTypeNode","decimals":9,"number":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#
        );
    }
}
