use crate::TypeNode;
use codama_nodes_derive::type_node;

#[type_node]
pub struct RemainderOptionTypeNode {
    // Children.
    pub item: TypeNode,
}

impl RemainderOptionTypeNode {
    pub fn new<T>(item: T) -> Self
    where
        T: Into<TypeNode>,
    {
        Self { item: item.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, U64};

    #[test]
    fn new() {
        let node = RemainderOptionTypeNode::new(NumberTypeNode::le(U64));
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
    }

    #[test]
    fn to_json() {
        let node = RemainderOptionTypeNode::new(NumberTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"remainderOptionTypeNode","item":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"remainderOptionTypeNode","item":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#;
        let node: RemainderOptionTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, RemainderOptionTypeNode::new(NumberTypeNode::le(U64)));
    }
}
