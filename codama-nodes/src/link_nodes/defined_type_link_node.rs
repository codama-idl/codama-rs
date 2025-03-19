use crate::{CamelCaseString, ProgramLinkNode};
use codama_nodes_derive::node;

#[node]
pub struct DefinedTypeLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program: Option<ProgramLinkNode>,
}

impl From<DefinedTypeLinkNode> for crate::Node {
    fn from(val: DefinedTypeLinkNode) -> Self {
        crate::Node::Link(val.into())
    }
}

impl DefinedTypeLinkNode {
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

impl From<String> for DefinedTypeLinkNode {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl From<&str> for DefinedTypeLinkNode {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = DefinedTypeLinkNode::new("my_type");
        assert_eq!(node.name, CamelCaseString::new("myType"));
    }

    #[test]
    fn new_from_program() {
        let node =
            DefinedTypeLinkNode::new_from_program("my_type", ProgramLinkNode::new("my_program"));
        assert_eq!(node.name, CamelCaseString::new("myType"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
    }

    #[test]
    fn from_string() {
        let node: DefinedTypeLinkNode = String::from("my_type").into();
        assert_eq!(node.name, CamelCaseString::new("myType"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn from_str() {
        let node: DefinedTypeLinkNode = "my_type".into();
        assert_eq!(node.name, CamelCaseString::new("myType"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn to_json() {
        let node = DefinedTypeLinkNode::new("myType");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"definedTypeLinkNode","name":"myType"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"definedTypeLinkNode","name":"myType"}"#;
        let node: DefinedTypeLinkNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, DefinedTypeLinkNode::new("myType"));
    }
}
