use crate::{CamelCaseString, ProgramLinkNode};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct AccountLinkNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub program: Option<ProgramLinkNode>,
}

impl AccountLinkNode {
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
        let node = AccountLinkNode::new("my_account");
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
    }

    #[test]
    fn new_from_program() {
        let node =
            AccountLinkNode::new_from_program("my_account", ProgramLinkNode::new("my_program"));
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
    }
}
