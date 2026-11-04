use crate::{Identifier, InstructionAccountLinkNode, InstructionLinkNode};

impl InstructionAccountLinkNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<Identifier>,
    {
        Self {
            identifier: identifier.into(),
            instruction: None,
            plugins: vec![],
        }
    }

    pub fn new_from_instruction<T>(identifier: T, instruction: InstructionLinkNode) -> Self
    where
        T: Into<Identifier>,
    {
        Self {
            identifier: identifier.into(),
            instruction: Some(instruction),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::id;
    use crate::ProgramLinkNode;

    use super::*;

    #[test]
    fn new() {
        let node = InstructionAccountLinkNode::new(id("myInstructionAccount"));
        assert_eq!(node.identifier, id("myInstructionAccount"));
    }

    #[test]
    fn new_from_instruction() {
        let node = InstructionAccountLinkNode::new_from_instruction(
            id("myInstructionAccount"),
            InstructionLinkNode::new_from_program(
                id("myInstruction"),
                ProgramLinkNode::new(id("myProgram")),
            ),
        );
        assert_eq!(node.identifier, id("myInstructionAccount"));
        assert_eq!(
            node.instruction,
            Some(InstructionLinkNode::new_from_program(
                id("myInstruction"),
                ProgramLinkNode::new(id("myProgram"))
            ))
        );
    }

    #[test]
    fn to_json() {
        let node = InstructionAccountLinkNode::new(id("myAccount"));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionAccountLinkNode","identifier":"myAccount"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionAccountLinkNode","identifier":"myAccount"}"#;
        let node: InstructionAccountLinkNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, InstructionAccountLinkNode::new(id("myAccount")));
    }
}
