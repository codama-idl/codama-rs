use crate::{CamelCaseString, PdaLinkNode, ProgramLinkNode};

impl PdaLinkNode {
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

impl From<String> for PdaLinkNode {
    fn from(identifier: String) -> Self {
        Self::new(identifier)
    }
}

impl From<&str> for PdaLinkNode {
    fn from(identifier: &str) -> Self {
        Self::new(identifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = PdaLinkNode::new("my_pda");
        assert_eq!(node.identifier, CamelCaseString::new("myPda"));
    }

    #[test]
    fn new_from_program() {
        let node = PdaLinkNode::new_from_program("my_pda", ProgramLinkNode::new("my_program"));
        assert_eq!(node.identifier, CamelCaseString::new("myPda"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
    }

    #[test]
    fn from_string() {
        let node: PdaLinkNode = String::from("my_pda").into();
        assert_eq!(node.identifier, CamelCaseString::new("myPda"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn from_str() {
        let node: PdaLinkNode = "my_pda".into();
        assert_eq!(node.identifier, CamelCaseString::new("myPda"));
        assert_eq!(node.program, None);
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
