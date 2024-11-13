use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug)]
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
