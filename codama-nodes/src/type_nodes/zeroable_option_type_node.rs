use crate::{ConstantValueNode, TypeNode};
use codama_nodes_derive::type_node;

#[type_node]
pub struct ZeroableOptionTypeNode {
    // Children.
    pub item: TypeNode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_value: Option<ConstantValueNode>,
}

impl Into<crate::Node> for ZeroableOptionTypeNode {
    fn into(self) -> crate::Node {
        crate::Node::Type(self.into())
    }
}

impl ZeroableOptionTypeNode {
    pub fn new<T>(item: T) -> Self
    where
        T: Into<TypeNode>,
    {
        Self {
            item: item.into(),
            zero_value: None,
        }
    }

    pub fn custom<T>(item: T, zero_value: ConstantValueNode) -> Self
    where
        T: Into<TypeNode>,
    {
        Self {
            item: item.into(),
            zero_value: Some(zero_value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base16, NumberTypeNode, U64};

    #[test]
    fn new() {
        let node = ZeroableOptionTypeNode::new(NumberTypeNode::le(U64));
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(node.zero_value, None);
    }

    #[test]
    fn custom() {
        let node = ZeroableOptionTypeNode::custom(
            NumberTypeNode::le(U64),
            ConstantValueNode::bytes(Base16, "ffffffffffffffff"),
        );
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
        assert_eq!(
            node.zero_value,
            Some(ConstantValueNode::bytes(Base16, "ffffffffffffffff"))
        );
    }

    #[test]
    fn to_json() {
        let node = ZeroableOptionTypeNode::new(NumberTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"zeroableOptionTypeNode","item":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"zeroableOptionTypeNode","item":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#;
        let node: ZeroableOptionTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ZeroableOptionTypeNode::new(NumberTypeNode::le(U64)));
    }
}
