use crate::{CamelCaseString, Node, ProvidedNode};

impl ProvidedNode {
    pub fn new<T, U>(identifier: T, node: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<Node>,
    {
        Self {
            identifier: identifier.into(),
            node: Box::new(node.into()),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerTypeNode, IntegerValueNode, U32};

    #[test]
    fn new() {
        let node = ProvidedNode::new("my_value", IntegerValueNode::new(42));
        assert_eq!(node.identifier, CamelCaseString::new("myValue"));
        assert_eq!(*node.node, Node::from(IntegerValueNode::new(42)));
    }

    #[test]
    fn to_json() {
        let node = ProvidedNode::new("myType", IntegerTypeNode::le(U32));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"providedNode","identifier":"myType","node":{"kind":"integerTypeNode","format":"u32","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"providedNode","identifier":"myType","node":{"kind":"integerTypeNode","format":"u32","endian":"le"}}"#;
        let node: ProvidedNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ProvidedNode::new("myType", IntegerTypeNode::le(U32)));
    }
}
