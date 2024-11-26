use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct ProgramLinkNode {
    // Data.
    pub name: CamelCaseString,
}

impl Into<crate::Node> for ProgramLinkNode {
    fn into(self) -> crate::Node {
        crate::Node::Link(self.into())
    }
}

impl ProgramLinkNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self { name: name.into() }
    }
}

impl From<String> for ProgramLinkNode {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl From<&str> for ProgramLinkNode {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = ProgramLinkNode::new("my_program");
        assert_eq!(node.name, CamelCaseString::new("myProgram"));
    }

    #[test]
    fn from_string() {
        let node: ProgramLinkNode = String::from("my_program").into();
        assert_eq!(node.name, CamelCaseString::new("myProgram"));
    }

    #[test]
    fn from_str() {
        let node: ProgramLinkNode = "my_program".into();
        assert_eq!(node.name, CamelCaseString::new("myProgram"));
    }

    #[test]
    fn to_json() {
        let node = ProgramLinkNode::new("myProgram");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"programLinkNode","name":"myProgram"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"programLinkNode","name":"myProgram"}"#;
        let node: ProgramLinkNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ProgramLinkNode::new("myProgram"));
    }
}
