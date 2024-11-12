use crate::Node;

use super::{NestedTypeNode, NumberTypeNode};

#[derive(Debug)]
pub struct SolAmountTypeNode {
    // Children.
    pub number: NestedTypeNode<NumberTypeNode>,
}

impl SolAmountTypeNode {
    pub fn new(number: NestedTypeNode<NumberTypeNode>) -> Self {
        Self { number }
    }
}

impl Node for SolAmountTypeNode {
    const KIND: &'static str = "solAmountTypeNode";
}
