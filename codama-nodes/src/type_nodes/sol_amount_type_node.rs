use crate::NodeTrait;

use super::{NestedTypeNode, NumberTypeNode, TypeNodeTrait};

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

impl TypeNodeTrait for SolAmountTypeNode {}
impl NodeTrait for SolAmountTypeNode {
    const KIND: &'static str = "solAmountTypeNode";
}
