use crate::Node;

use super::{NestedTypeNode, NumberTypeNode};

#[derive(Debug)]
pub struct SolAmountTypeNode {
    // Children.
    pub number: NestedTypeNode<NumberTypeNode>,
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

impl Node for SolAmountTypeNode {
    const KIND: &'static str = "solAmountTypeNode";
}
