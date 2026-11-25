use crate::{InstructionInputValueNode, Node, ValueNode};
use codama_errors::CodamaError;

/// The spec union `instructionInputValueNode` includes `valueNode`, so every value node is a member of
/// it. Going through `Node` keeps this true whatever value nodes the spec adds.
impl From<ValueNode> for InstructionInputValueNode {
    fn from(value: ValueNode) -> Self {
        Self::try_from(Node::from(value)).expect("every value node is a member of the union")
    }
}

/// The value nodes among the instruction input values. The contextual values and links are not value
/// nodes, so they fail with `InvalidNodeConversion`.
impl TryFrom<InstructionInputValueNode> for ValueNode {
    type Error = CodamaError;

    fn try_from(value: InstructionInputValueNode) -> Result<Self, Self::Error> {
        Self::try_from(Node::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerValueNode, Node, PayerValueNode, ProgramLinkNode};

    #[test]
    fn into_node() {
        for (value, expected) in [
            (
                InstructionInputValueNode::from(PayerValueNode::new()),
                Node::PayerValue(PayerValueNode::new()),
            ),
            (
                IntegerValueNode::new(42).into(),
                Node::IntegerValue(IntegerValueNode::new(42)),
            ),
            (
                ProgramLinkNode::new("system").into(),
                Node::ProgramLink(ProgramLinkNode::new("system")),
            ),
        ] {
            assert_eq!(Node::from(value), expected);
        }
    }
}
