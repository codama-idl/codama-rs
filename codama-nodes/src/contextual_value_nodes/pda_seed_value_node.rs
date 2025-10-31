use crate::{
    AccountValueNode, ArgumentValueNode, ArrayValueNode, BooleanValueNode, BytesValueNode,
    CamelCaseString, ConstantValueNode, EnumValueNode, HasName, MapValueNode, NoneValueNode,
    NumberValueNode, PublicKeyValueNode, SetValueNode, SomeValueNode, StringValueNode,
    StructValueNode, TupleValueNode, ValueNode,
};
use codama_nodes_derive::{node, node_union};

#[node]
pub struct PdaSeedValueNode {
    // Data.
    pub name: CamelCaseString,

    // Children.
    pub value: PdaSeedValueValueNode,
}

impl From<PdaSeedValueNode> for crate::Node {
    fn from(val: PdaSeedValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl PdaSeedValueNode {
    pub fn new<T, U>(name: T, value: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<PdaSeedValueValueNode>,
    {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl HasName for PdaSeedValueNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}

#[node_union]
pub enum PdaSeedValueValueNode {
    Account(AccountValueNode),
    Argument(ArgumentValueNode),

    // ValueNodes.
    Array(ArrayValueNode),
    Boolean(BooleanValueNode),
    Bytes(BytesValueNode),
    Constant(ConstantValueNode),
    Enum(EnumValueNode),
    Map(MapValueNode),
    None(NoneValueNode),
    Number(NumberValueNode),
    PublicKey(PublicKeyValueNode),
    Set(SetValueNode),
    Some(SomeValueNode),
    String(StringValueNode),
    Struct(StructValueNode),
    Tuple(TupleValueNode),
}

impl From<ValueNode> for PdaSeedValueValueNode {
    fn from(value: ValueNode) -> Self {
        match value {
            ValueNode::Array(value) => Self::Array(value),
            ValueNode::Boolean(value) => Self::Boolean(value),
            ValueNode::Bytes(value) => Self::Bytes(value),
            ValueNode::Constant(value) => Self::Constant(value),
            ValueNode::Enum(value) => Self::Enum(value),
            ValueNode::Map(value) => Self::Map(value),
            ValueNode::None(value) => Self::None(value),
            ValueNode::Number(value) => Self::Number(value),
            ValueNode::PublicKey(value) => Self::PublicKey(value),
            ValueNode::Set(value) => Self::Set(value),
            ValueNode::Some(value) => Self::Some(value),
            ValueNode::String(value) => Self::String(value),
            ValueNode::Struct(value) => Self::Struct(value),
            ValueNode::Tuple(value) => Self::Tuple(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::NumberValueNode;

    use super::*;

    #[test]
    fn new() {
        let node = PdaSeedValueNode::new("answer", NumberValueNode::new(42));
        assert_eq!(node.name, CamelCaseString::from("answer"));
        assert_eq!(
            node.value,
            PdaSeedValueValueNode::Number(NumberValueNode::new(42))
        );
    }

    #[test]
    fn to_json() {
        let node = PdaSeedValueNode::new("answer", NumberValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"pdaSeedValueNode","name":"answer","value":{"kind":"numberValueNode","number":42}}"#
        );
    }

    #[test]
    fn from_json() {
        let json: &str = r#"{"kind":"pdaSeedValueNode","name":"answer","value":{"kind":"numberValueNode","number":42}}"#;
        let node: PdaSeedValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("answer", NumberValueNode::new(42u32))
        );
    }
}
