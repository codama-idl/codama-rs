use crate::{CamelCaseString, InstructionLinkNode, ProgramLinkNode};

impl InstructionLinkNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            identifier: identifier.into(),
            program: None,
            plugins: vec![],
        }
    }

    pub fn new_from_program<T>(identifier: T, program: ProgramLinkNode) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            identifier: identifier.into(),
            program: Some(program),
            plugins: vec![],
        }
    }
}

impl From<String> for InstructionLinkNode {
    fn from(identifier: String) -> Self {
        Self::new(identifier)
    }
}

impl From<&str> for InstructionLinkNode {
    fn from(identifier: &str) -> Self {
        Self::new(identifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = InstructionLinkNode::new("my_instruction");
        assert_eq!(node.identifier, CamelCaseString::new("myInstruction"));
    }

    #[test]
    fn new_from_program() {
        let node = InstructionLinkNode::new_from_program(
            "my_instruction",
            ProgramLinkNode::new("my_program"),
        );
        assert_eq!(node.identifier, CamelCaseString::new("myInstruction"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
    }

    #[test]
    fn from_string() {
        let node: InstructionLinkNode = String::from("my_instruction").into();
        assert_eq!(node.identifier, CamelCaseString::new("myInstruction"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn from_str() {
        let node: InstructionLinkNode = "my_instruction".into();
        assert_eq!(node.identifier, CamelCaseString::new("myInstruction"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn to_json() {
        let node = InstructionLinkNode::new("myInstruction");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionLinkNode","identifier":"myInstruction"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionLinkNode","identifier":"myInstruction"}"#;
        let node: InstructionLinkNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, InstructionLinkNode::new("myInstruction"));
    }
}
