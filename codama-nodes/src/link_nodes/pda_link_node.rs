use crate::{Identifier, PdaLinkNode, ProgramLinkNode};

impl PdaLinkNode {
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
        let node = PdaLinkNode::new("myPda");
        assert_eq!(node.identifier, Identifier::from("myPda"));
    }

    #[test]
    fn new_from_program() {
        let node = PdaLinkNode::new_from_program("myPda", ProgramLinkNode::new("myProgram"));
        assert_eq!(node.identifier, Identifier::from("myPda"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
    }

    #[test]
    fn to_json() {
        let node = PdaLinkNode::new("myPda");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"pdaLinkNode","identifier":"myPda"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"pdaLinkNode","identifier":"myPda"}"#;
        let node: PdaLinkNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PdaLinkNode::new("myPda"));
    }
}
