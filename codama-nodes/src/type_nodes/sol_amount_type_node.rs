use crate::{NestedTypeNode, NumberTypeNode};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Debug, Node, TypeNode)]
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
