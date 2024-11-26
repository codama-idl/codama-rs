use crate::TypeNode;
use codama_nodes_derive::type_node;

#[type_node]
pub struct TupleTypeNode {
    // Children.
    pub items: Vec<TypeNode>,
}

impl Into<crate::Node> for TupleTypeNode {
    fn into(self) -> crate::Node {
        crate::Node::Type(self.into())
    }
}

impl TupleTypeNode {
    pub fn new(items: Vec<TypeNode>) -> Self {
        Self { items }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, StringTypeNode, U32};

    #[test]
    fn new() {
        let node = TupleTypeNode::new(vec![
            NumberTypeNode::le(U32).into(),
            StringTypeNode::utf8().into(),
        ]);
        assert_eq!(
            node.items,
            vec![
                TypeNode::Number(NumberTypeNode::le(U32)),
                TypeNode::String(StringTypeNode::utf8())
            ]
        );
    }

    #[test]
    fn to_json() {
        let node = TupleTypeNode::new(vec![
            NumberTypeNode::le(U32).into(),
            StringTypeNode::utf8().into(),
        ]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"tupleTypeNode","items":[{"kind":"numberTypeNode","format":"u32","endian":"le"},{"kind":"stringTypeNode","encoding":"utf8"}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"tupleTypeNode","items":[{"kind":"numberTypeNode","format":"u32","endian":"le"},{"kind":"stringTypeNode","encoding":"utf8"}]}"#;
        let node: TupleTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            TupleTypeNode::new(vec![
                NumberTypeNode::le(U32).into(),
                StringTypeNode::utf8().into(),
            ])
        );
    }
}
