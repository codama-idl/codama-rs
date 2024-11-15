use crate::{CamelCaseString, ProgramLinkNode};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct InstructionLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub program: Option<ProgramLinkNode>,
}

impl InstructionLinkNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            program: None,
        }
    }

    pub fn new_from_program<T>(name: T, program: ProgramLinkNode) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            program: Some(program),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = InstructionLinkNode::new("my_instruction");
        assert_eq!(node.name, CamelCaseString::new("myInstruction"));
    }

    #[test]
    fn new_from_program() {
        let node = InstructionLinkNode::new_from_program(
            "my_instruction",
            ProgramLinkNode::new("my_program"),
        );
        assert_eq!(node.name, CamelCaseString::new("myInstruction"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
    }
}
