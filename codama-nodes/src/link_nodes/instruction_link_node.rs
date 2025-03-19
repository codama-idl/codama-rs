use crate::{CamelCaseString, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
pub struct InstructionLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
}

impl From<InstructionLinkNode> for crate::Node {
    fn from(val: InstructionLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
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

impl From<String> for InstructionLinkNode {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl From<&str> for InstructionLinkNode {
    fn from(name: &str) -> Self {
        Self::new(name)
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

    #[test]
    fn from_string() {
        let node: InstructionLinkNode = String::from("my_instruction").into();
        assert_eq!(node.name, CamelCaseString::new("myInstruction"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn from_str() {
        let node: InstructionLinkNode = "my_instruction".into();
        assert_eq!(node.name, CamelCaseString::new("myInstruction"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn to_json() {
        let node = InstructionLinkNode::new("myInstruction");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionLinkNode","name":"myInstruction"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionLinkNode","name":"myInstruction"}"#;
        let node: InstructionLinkNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, InstructionLinkNode::new("myInstruction"));
    }
}
