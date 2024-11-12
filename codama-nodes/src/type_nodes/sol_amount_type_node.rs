use crate::Node;

use super::NumberTypeNodeFlag;

#[derive(Debug)]
pub struct SolAmountTypeNode<T: NumberTypeNodeFlag> {
    // Children.
    pub number: T,
}

impl<T: NumberTypeNodeFlag> SolAmountTypeNode<T> {
    pub fn new(number: T) -> Self {
        Self { number }
    }
}

impl<T: NumberTypeNodeFlag> Node for SolAmountTypeNode<T> {
    const KIND: &'static str = "solAmountTypeNode";
}
