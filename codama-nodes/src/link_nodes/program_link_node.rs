use crate::CamelCaseString;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct ProgramLinkNode {
    // Data.
    pub name: CamelCaseString,
}

impl ProgramLinkNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self { name: name.into() }
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
}
