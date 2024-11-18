use crate::{CamelCaseString, Docs, TypeNode};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct VariablePdaSeedNode {
    // Data.
    pub name: CamelCaseString,
    pub docs: Docs,

    // Children.
    pub r#type: TypeNode,
}

impl VariablePdaSeedNode {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DefaultValueStrategy {
    Omitted,
    Optional,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, U32};

    #[test]
    fn new() {
        let node = VariablePdaSeedNode::new("my_seed", NumberTypeNode::le(U32));
        assert_eq!(node.name, CamelCaseString::new("mySeed"));
        assert_eq!(node.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
    }

    #[test]
    fn direct_instantiation() {
        let node = VariablePdaSeedNode {
            name: "mySeed".into(),
            docs: vec!["Hello".to_string()].into(),
            r#type: NumberTypeNode::le(U32).into(),
        };
        assert_eq!(node.name, CamelCaseString::new("mySeed"));
        assert_eq!(*node.docs, vec!["Hello".to_string()]);
        assert_eq!(node.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
    }
}
