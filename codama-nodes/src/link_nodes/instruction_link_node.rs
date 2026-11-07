use crate::{Identifier, InstructionLinkNode, ProgramLinkNode};

impl InstructionLinkNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<Identifier>,
    {
        Self {
            identifier: identifier.into(),
            program: None,
            plugins: vec![],
        }
    }

    pub fn new_from_program<T>(identifier: T, program: ProgramLinkNode) -> Self
    where
        T: Into<Identifier>,
    {
        Self {
            identifier: identifier.into(),
            program: Some(program),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Identifier;

    #[test]
    fn new() {
        let node = InstructionLinkNode::new("myInstruction");
        assert_eq!(node.identifier, Identifier::from("myInstruction"));
    }

    #[test]
    fn new_from_program() {
        let node = InstructionLinkNode::new_from_program(
            "myInstruction",
            ProgramLinkNode::new("myProgram"),
        );
        assert_eq!(node.identifier, Identifier::from("myInstruction"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
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
