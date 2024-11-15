use crate::{CamelCaseString, InstructionLinkNode};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct InstructionAccountLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub instruction: Option<InstructionLinkNode>,
}

impl InstructionAccountLinkNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            instruction: None,
        }
    }

    pub fn new_from_instruction<T>(name: T, instruction: InstructionLinkNode) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            instruction: Some(instruction),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ProgramLinkNode;

    use super::*;

    #[test]
    fn new() {
        let node = InstructionAccountLinkNode::new("my_instruction_account");
        assert_eq!(node.name, CamelCaseString::new("myInstructionAccount"));
    }

    #[test]
    fn new_from_instruction() {
        let node = InstructionAccountLinkNode::new_from_instruction(
            "my_instruction_account",
            InstructionLinkNode::new_from_program(
                "my_instruction",
                ProgramLinkNode::new("my_program"),
            ),
        );
        assert_eq!(node.name, CamelCaseString::new("myInstructionAccount"));
        assert_eq!(
            node.instruction,
            Some(InstructionLinkNode::new_from_program(
                "myInstruction",
                ProgramLinkNode::new("myProgram")
            ))
        );
    }
}
