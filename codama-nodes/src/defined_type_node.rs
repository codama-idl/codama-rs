use crate::{CamelCaseString, Docs, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct DefinedTypeNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default)]
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

    #[test]
    fn to_json() {
        let node = DefinedTypeNode::new("myType", NumberTypeNode::le(U8));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"definedTypeNode","name":"myType","type":{"kind":"numberTypeNode","format":"u8","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"definedTypeNode","name":"myType","type":{"kind":"numberTypeNode","format":"u8","endian":"le"}}"#;
        let node: DefinedTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, DefinedTypeNode::new("myType", NumberTypeNode::le(U8)));
    }
}
