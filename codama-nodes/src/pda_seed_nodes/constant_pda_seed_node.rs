use crate::{ConstantPdaSeedNode, ConstantPdaSeedValue, Node, TypeNode, ValueNode};

impl ConstantPdaSeedNode {
    pub fn new<T, U>(r#type: T, value: U) -> Self
    where
        T: Into<TypeNode>,
        U: Into<ConstantPdaSeedValue>,
    {
        Self {
            r#type: Box::new(r#type.into()),
            value: Box::new(value.into()),
            plugins: vec![],
        }
    }
}

/// The spec union `constantPdaSeedValue` is `programIdValueNode | valueNode`, so every value node is a
/// member of it. Going through `Node` keeps this true whatever value nodes the spec adds.
impl From<ValueNode> for ConstantPdaSeedValue {
    fn from(value: ValueNode) -> Self {
        Self::try_from(Node::from(value)).expect("every value node is a member of the union")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FloatValueNode, IntegerTypeNode, IntegerValueNode, U64};

    #[test]
    fn new() {
        let node = ConstantPdaSeedNode::new(IntegerTypeNode::le(U64), IntegerValueNode::new(42u64));
        assert_eq!(*node.r#type, TypeNode::Integer(IntegerTypeNode::le(U64)));
        assert_eq!(
            *node.value,
            ConstantPdaSeedValue::IntegerValue(IntegerValueNode::new(42u64))
        );
    }

    #[test]
    fn from_value_node() {
        let value: ValueNode = FloatValueNode::new(1.5).into();
        assert_eq!(
            ConstantPdaSeedValue::from(value),
            ConstantPdaSeedValue::FloatValue(FloatValueNode::new(1.5))
        );
    }

    #[test]
    fn to_json() {
        let node = ConstantPdaSeedNode::new(IntegerTypeNode::le(U64), IntegerValueNode::new(42u64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"constantPdaSeedNode","type":{"kind":"integerTypeNode","format":"u64","endian":"le"},"value":{"kind":"integerValueNode","value":"42"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"constantPdaSeedNode","type":{"kind":"integerTypeNode","format":"u64","endian":"le"},"value":{"kind":"integerValueNode","value":"42"}}"#;
        let node: ConstantPdaSeedNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ConstantPdaSeedNode::new(IntegerTypeNode::le(U64), IntegerValueNode::new(42u64))
        );
    }
}
