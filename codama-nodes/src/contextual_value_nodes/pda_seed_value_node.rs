use crate::{
    AccountValueNode, ArgumentValueNode, ArrayValueNode, BooleanValueNode, BytesValueNode,
    CamelCaseString, ConstantValueNode, EnumValueNode, MapValueNode, NoneValueNode,
    NumberValueNode, PublicKeyValueNode, SetValueNode, SomeValueNode, StringValueNode,
    StructValueNode, TupleValueNode,
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
