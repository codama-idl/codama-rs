use crate::{AccountLinkNode, ArgumentValueNode, NumberValueNode, ResolverValueNode};
use codama_nodes_derive::{IntoEnum, Node};

#[derive(Node, Debug, PartialEq)]
pub struct InstructionByteDeltaNode {
    // Data.
    pub with_header: bool,
    pub substract: bool,

    // Children.
    pub value: InstructionByteDeltaNodeValue,
}

impl InstructionByteDeltaNode {
    pub fn new<T>(value: T, with_header: bool) -> Self
    where
        T: Into<InstructionByteDeltaNodeValue>,
    {
        Self {
            value: value.into(),
            with_header,
            substract: false,
        }
    }

    pub fn minus<T>(value: T, with_header: bool) -> Self
    where
        T: Into<InstructionByteDeltaNodeValue>,
    {
        Self {
            value: value.into(),
            with_header,
            substract: true,
        }
    }
}

#[derive(IntoEnum, Debug, PartialEq)]
pub enum InstructionByteDeltaNodeValue {
    Account(AccountLinkNode),
    Argument(ArgumentValueNode),
    Number(NumberValueNode),
    Resolver(ResolverValueNode),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = InstructionByteDeltaNode::new(ArgumentValueNode::new("myArgument"), true);
        assert_eq!(
            node.value,
            InstructionByteDeltaNodeValue::Argument(ArgumentValueNode::new("myArgument"))
        );
        assert_eq!(node.with_header, true);
        assert_eq!(node.substract, false);
    }

    #[test]
    fn minus() {
        let node = InstructionByteDeltaNode::minus(NumberValueNode::new(42), true);
        assert_eq!(
            node.value,
            InstructionByteDeltaNodeValue::Number(NumberValueNode::new(42))
        );
        assert_eq!(node.with_header, true);
        assert_eq!(node.substract, true);
    }
}
