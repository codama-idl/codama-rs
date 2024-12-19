use crate::{BytesEncoding, BytesTypeNode, BytesValueNode, TypeNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct ConstantValueNode {
    // Children.
    pub r#type: Box<TypeNode>,
    pub value: Box<ValueNode>,
}

impl From<ConstantValueNode> for crate::Node {
    fn from(val: ConstantValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}

impl ConstantValueNode {
    pub fn new<T, U>(r#type: T, value: U) -> Self
    where
        T: Into<TypeNode>,
        U: Into<ValueNode>,
    {
        Self {
            r#type: Box::new(r#type.into()),
            value: Box::new(value.into()),
        }
    }

    pub fn bytes<T>(encoding: BytesEncoding, data: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            r#type: Box::new(BytesTypeNode::new().into()),
            value: Box::new(BytesValueNode::new(encoding, data).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base16, NumberTypeNode, NumberValueNode, U64};

    #[test]
    fn new() {
        let node = ConstantValueNode::new(NumberTypeNode::le(U64), NumberValueNode::new(42u64));
        assert_eq!(*node.r#type, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(*node.value, ValueNode::Number(NumberValueNode::new(42u64)));
    }

    #[test]
    fn bytes() {
        let node = ConstantValueNode::bytes(Base16, "deadb0d1e5");
        assert_eq!(*node.r#type, TypeNode::Bytes(BytesTypeNode::new()));
        assert_eq!(
            *node.value,
            ValueNode::Bytes(BytesValueNode::base16("deadb0d1e5"))
        );
    }

    #[test]
    fn to_json() {
        let node = ConstantValueNode::bytes(Base16, "deadb0d1e5");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"deadb0d1e5","encoding":"base16"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"deadb0d1e5","encoding":"base16"}}"#;
        let node: ConstantValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ConstantValueNode::bytes(Base16, "deadb0d1e5"));
    }
}
