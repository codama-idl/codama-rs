use crate::{TypeNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct ConstantPdaSeedNode {
    // Children.
    pub r#type: TypeNode,
    pub value: ValueNode,
}

impl ConstantPdaSeedNode {
    pub fn new<T, U>(r#type: T, value: U) -> Self
    where
        T: Into<TypeNode>,
        U: Into<ValueNode>,
    {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, NumberValueNode, U64};

    #[test]
    fn new() {
        let node = ConstantPdaSeedNode::new(NumberTypeNode::le(U64), NumberValueNode::new(42u64));
        assert_eq!(node.r#type, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.value, ValueNode::Number(NumberValueNode::new(42u64)));
    }

    #[test]
    fn to_json() {
        let node = ConstantPdaSeedNode::new(NumberTypeNode::le(U64), NumberValueNode::new(42u64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"constantPdaSeedNode","type":{"kind":"numberTypeNode","format":"u64","endian":"le"},"value":{"kind":"numberValueNode","number":42}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"constantPdaSeedNode","type":{"kind":"numberTypeNode","format":"u64","endian":"le"},"value":{"kind":"numberValueNode","number":42}}"#;
        let node: ConstantPdaSeedNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ConstantPdaSeedNode::new(NumberTypeNode::le(U64), NumberValueNode::new(42u64))
        );
    }
}
