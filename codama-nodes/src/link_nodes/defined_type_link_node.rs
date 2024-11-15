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
}
