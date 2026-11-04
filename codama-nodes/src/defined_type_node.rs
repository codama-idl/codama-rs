use crate::{DefinedTypeNode, Docs, Identifier, TypeNode};

impl DefinedTypeNode {
    pub fn new<T, U>(identifier: T, r#type: U) -> Self
    where
        T: Into<Identifier>,
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
    use crate::test_utils::id;
    use crate::{IntegerTypeNode, U8};

    #[test]
    fn new() {
        let node = DefinedTypeNode::new(id("myType"), IntegerTypeNode::le(U8));
        assert_eq!(node.identifier, id("myType"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(*node.r#type, TypeNode::Integer(IntegerTypeNode::le(U8)));
    }

    #[test]
    fn direct_instantiation() {
        let node = DefinedTypeNode {
            identifier: id("myType"),
            docs: Docs::default(),
            r#type: Box::new(IntegerTypeNode::le(U8).into()),
            plugins: vec![],
        };
        assert_eq!(node.identifier, id("myType"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(*node.r#type, TypeNode::Integer(IntegerTypeNode::le(U8)));
    }

    #[test]
    fn to_json() {
        let node = DefinedTypeNode::new(id("myType"), IntegerTypeNode::le(U8));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"definedTypeNode","identifier":"myType","type":{"kind":"integerTypeNode","format":"u8","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"definedTypeNode","identifier":"myType","type":{"kind":"integerTypeNode","format":"u8","endian":"le"}}"#;
        let node: DefinedTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            DefinedTypeNode::new(id("myType"), IntegerTypeNode::le(U8))
        );
    }
}
