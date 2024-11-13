use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct AmountTypeNode {
    // Data.
    pub decimals: u8,
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
    use crate::{Endian, NumberTypeNode, U64};

    #[test]
    fn new() {
        let node = AmountTypeNode::new(
            NumberTypeNode::new(U64, Endian::Big),
            8,
            Some("SOL".to_string()),
        );
        assert_eq!(
            node.number,
            NestedTypeNode::Value(NumberTypeNode::new(U64, Endian::Big))
        );
        assert_eq!(node.decimals, 8);
        assert_eq!(node.unit, Some("SOL".to_string()));
    }
}
