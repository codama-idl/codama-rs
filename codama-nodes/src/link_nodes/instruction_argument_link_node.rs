use crate::{CamelCaseString, InstructionLinkNode};
use codama_nodes_derive::node;

#[node]
pub struct InstructionArgumentLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub instruction: Option<InstructionLinkNode>,
}

impl From<InstructionArgumentLinkNode> for crate::Node {
    fn from(val: InstructionArgumentLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl InstructionArgumentLinkNode {
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

impl From<String> for InstructionArgumentLinkNode {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl From<&str> for InstructionArgumentLinkNode {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

#[cfg(test)]
mod tests {
    use crate::ProgramLinkNode;

    use super::*;

    #[test]
    fn new() {
        let node = InstructionArgumentLinkNode::new("my_instruction_argument");
        assert_eq!(node.name, CamelCaseString::new("myInstructionArgument"));
    }

    #[test]
    fn new_from_instruction() {
        let node = InstructionArgumentLinkNode::new_from_instruction(
            "my_instruction_argument",
            InstructionLinkNode::new_from_program(
                "my_instruction",
                ProgramLinkNode::new("my_program"),
            ),
        );
        assert_eq!(node.name, CamelCaseString::new("myInstructionArgument"));
        assert_eq!(
            node.instruction,
            Some(InstructionLinkNode::new_from_program(
                "myInstruction",
                ProgramLinkNode::new("myProgram")
            ))
        );
    }

    #[test]
    fn from_string() {
        let node: InstructionArgumentLinkNode = String::from("my_instruction_argument").into();
        assert_eq!(node.name, CamelCaseString::new("myInstructionArgument"));
        assert_eq!(node.instruction, None);
    }

    #[test]
    fn from_str() {
        let node: InstructionArgumentLinkNode = "my_instruction_argument".into();
        assert_eq!(node.name, CamelCaseString::new("myInstructionArgument"));
        assert_eq!(node.instruction, None);
    }

    #[test]
    fn to_json() {
        let node = InstructionArgumentLinkNode::new("myArgument");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionArgumentLinkNode","name":"myArgument"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionArgumentLinkNode","name":"myArgument"}"#;
        let node: InstructionArgumentLinkNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, InstructionArgumentLinkNode::new("myArgument"));
    }
}
