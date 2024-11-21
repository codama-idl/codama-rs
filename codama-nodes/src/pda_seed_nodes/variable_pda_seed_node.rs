use crate::{CamelCaseString, Docs, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct VariablePdaSeedNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default)]
    #[serde(skip_serializing_if = "Docs::is_empty")]
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

    #[test]
    fn to_json() {
        let node = VariablePdaSeedNode::new("mySeed", NumberTypeNode::le(U32));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"variablePdaSeedNode","name":"mySeed","type":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"variablePdaSeedNode","name":"mySeed","type":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#;
        let node: VariablePdaSeedNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            VariablePdaSeedNode::new("mySeed", NumberTypeNode::le(U32))
        );
    }
}
