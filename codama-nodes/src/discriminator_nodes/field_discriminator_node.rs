use crate::CamelCaseString;
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct FieldDiscriminatorNode {
    // Data.
    pub name: CamelCaseString,
    pub offset: usize,
}

impl FieldDiscriminatorNode {
    pub fn new<T>(name: T, offset: usize) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = FieldDiscriminatorNode::new("my_field", 0);
        assert_eq!(node.name, CamelCaseString::new("myField"));
        assert_eq!(node.offset, 0);
    }
}
