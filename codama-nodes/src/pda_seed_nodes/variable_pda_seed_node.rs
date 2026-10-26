use crate::{CamelCaseString, Docs, TypeNode, VariablePdaSeedNode};

impl VariablePdaSeedNode {
    pub fn new<T, U>(identifier: T, r#type: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<TypeNode>,
    {
        Self {
            identifier: identifier.into(),
            docs: Docs::default(),
            r#type: Box::new(r#type.into()),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerTypeNode, Text, U32};

    #[test]
    fn new() {
        let node = VariablePdaSeedNode::new("my_seed", IntegerTypeNode::le(U32));
        assert_eq!(node.identifier, CamelCaseString::new("mySeed"));
        assert_eq!(*node.r#type, TypeNode::Integer(IntegerTypeNode::le(U32)));
    }

    #[test]
    fn direct_instantiation() {
        let node = VariablePdaSeedNode {
            identifier: "mySeed".into(),
            docs: vec!["Hello".to_string()].into(),
            r#type: Box::new(IntegerTypeNode::le(U32).into()),
            plugins: vec![],
        };
        assert_eq!(node.identifier, CamelCaseString::new("mySeed"));
        assert_eq!(*node.docs, Some(Text::from("Hello")));
        assert_eq!(*node.r#type, TypeNode::Integer(IntegerTypeNode::le(U32)));
    }

    #[test]
    fn to_json() {
        let node = VariablePdaSeedNode::new("mySeed", IntegerTypeNode::le(U32));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"variablePdaSeedNode","identifier":"mySeed","type":{"kind":"integerTypeNode","format":"u32","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"variablePdaSeedNode","identifier":"mySeed","type":{"kind":"integerTypeNode","format":"u32","endian":"le"}}"#;
        let node: VariablePdaSeedNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            VariablePdaSeedNode::new("mySeed", IntegerTypeNode::le(U32))
        );
    }
}
