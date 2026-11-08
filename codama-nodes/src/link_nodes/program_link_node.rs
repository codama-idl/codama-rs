use crate::{Identifier, ProgramLinkNode};

impl ProgramLinkNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<Identifier>,
    {
        Self {
            identifier: identifier.into(),
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
        let node = ProgramLinkNode::new("myProgram");
        assert_eq!(node.identifier, Identifier::from("myProgram"));
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
