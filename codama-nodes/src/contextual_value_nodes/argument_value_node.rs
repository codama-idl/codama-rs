use crate::CamelCaseString;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct ArgumentValueNode {
    // Data.
    pub name: CamelCaseString,
}

impl ArgumentValueNode {
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
        let node = ArgumentValueNode::new("my_argument");
        assert_eq!(node.name, CamelCaseString::new("myArgument"));
    }
}
