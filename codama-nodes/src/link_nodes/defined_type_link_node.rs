use crate::{CamelCaseString, ProgramLinkNode};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct DefinedTypeLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub program: Option<ProgramLinkNode>,
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
}
