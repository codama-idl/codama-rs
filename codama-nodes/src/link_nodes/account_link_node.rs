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

impl From<String> for AccountLinkNode {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl From<&str> for AccountLinkNode {
    fn from(name: &str) -> Self {
        Self::new(name)
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

    #[test]
    fn from_string() {
        let node: AccountLinkNode = String::from("my_account").into();
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn from_str() {
        let node: AccountLinkNode = "my_account".into();
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
        assert_eq!(node.program, None);
    }
}
