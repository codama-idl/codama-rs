use crate::{CamelCaseString, Node, ProvidedNode};

impl ProvidedNode {
    pub fn new<T, U>(name: T, node: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<Node>,
    {
        Self {
            name: name.into(),
            node: Box::new(node.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, NumberValueNode, U32};

    #[test]
    fn new() {
        let node = ProvidedNode::new("my_value", NumberValueNode::new(42));
        assert_eq!(node.name, CamelCaseString::new("myValue"));
        assert_eq!(*node.node, Node::from(NumberValueNode::new(42)));
    }

    #[test]
    fn to_json() {
        let node = ProvidedNode::new("myType", NumberTypeNode::le(U32));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"providedNode","name":"myType","node":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"providedNode","name":"myType","node":{"kind":"numberTypeNode","format":"u32","endian":"le"}}"#;
        let node: ProvidedNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ProvidedNode::new("myType", NumberTypeNode::le(U32)));
    }
}
