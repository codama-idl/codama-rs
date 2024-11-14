use crate::CamelCaseString;
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct EnumEmptyVariantTypeNode {
    // Data.
    pub name: CamelCaseString,
    pub discriminator: Option<usize>,
}

impl EnumEmptyVariantTypeNode {
    pub fn new<T>(name: T, discriminator: Option<usize>) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            discriminator,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = EnumEmptyVariantTypeNode::new("my_variant", Some(42));
        assert_eq!(node.name, CamelCaseString::new("myVariant"));
        assert_eq!(node.discriminator, Some(42));
    }
}
