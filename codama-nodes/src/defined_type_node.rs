use crate::{CamelCaseString, Docs, TypeNode};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct DefinedTypeNode {
    // Data.
    pub name: CamelCaseString,
    pub docs: Docs,

    // Children.
    pub r#type: TypeNode,
}

impl DefinedTypeNode {
    pub fn new<T, U>(name: T, r#type: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<TypeNode>,
    {
        Self {
            name: name.into(),
            docs: Docs::default(),
            r#type: r#type.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, U8};

    #[test]
    fn new() {
        let node = DefinedTypeNode::new("myType", NumberTypeNode::le(U8));
        assert_eq!(node.name, CamelCaseString::new("myType"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.r#type, TypeNode::Number(NumberTypeNode::le(U8)));
    }

    #[test]
    fn direct_instantiation() {
        let node = DefinedTypeNode {
            name: "myType".into(),
            docs: Docs::default(),
            r#type: NumberTypeNode::le(U8).into(),
        };
        assert_eq!(node.name, CamelCaseString::new("myType"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.r#type, TypeNode::Number(NumberTypeNode::le(U8)));
    }
}
