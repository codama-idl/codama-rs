use crate::{CamelCaseString, ProgramLinkNode};

impl ProgramLinkNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            identifier: identifier.into(),
            plugins: vec![],
        }
    }
}

impl From<String> for ProgramLinkNode {
    fn from(identifier: String) -> Self {
        Self::new(identifier)
    }
}

impl From<&str> for ProgramLinkNode {
    fn from(identifier: &str) -> Self {
        Self::new(identifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = ProgramLinkNode::new("my_program");
        assert_eq!(node.identifier, CamelCaseString::new("myProgram"));
    }

    #[test]
    fn from_string() {
        let node: ProgramLinkNode = String::from("my_program").into();
        assert_eq!(node.identifier, CamelCaseString::new("myProgram"));
    }

    #[test]
    fn from_str() {
        let node: ProgramLinkNode = "my_program".into();
        assert_eq!(node.identifier, CamelCaseString::new("myProgram"));
    }

    #[test]
    fn to_json() {
        let node = ProgramLinkNode::new("myProgram");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"programLinkNode","identifier":"myProgram"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"programLinkNode","identifier":"myProgram"}"#;
        let node: ProgramLinkNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ProgramLinkNode::new("myProgram"));
    }
}
