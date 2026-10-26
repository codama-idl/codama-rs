use crate::{TupleTypeNode, TypeNode};

impl TupleTypeNode {
    pub fn new(items: Vec<TypeNode>) -> Self {
        Self {
            items,
            transforms: vec![],
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerTypeNode, StringTypeNode, U32};

    #[test]
    fn new() {
        let node = TupleTypeNode::new(vec![
            IntegerTypeNode::le(U32).into(),
            StringTypeNode::utf8().into(),
        ]);
        assert_eq!(
            node.items,
            vec![
                TypeNode::Integer(IntegerTypeNode::le(U32)),
                TypeNode::String(StringTypeNode::utf8())
            ]
        );
    }

    #[test]
    fn to_json() {
        let node = TupleTypeNode::new(vec![
            IntegerTypeNode::le(U32).into(),
            StringTypeNode::utf8().into(),
        ]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"tupleTypeNode","items":[{"kind":"integerTypeNode","format":"u32","endian":"le"},{"kind":"stringTypeNode","encoding":"utf8"}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"tupleTypeNode","items":[{"kind":"integerTypeNode","format":"u32","endian":"le"},{"kind":"stringTypeNode","encoding":"utf8"}]}"#;
        let node: TupleTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            TupleTypeNode::new(vec![
                IntegerTypeNode::le(U32).into(),
                StringTypeNode::utf8().into(),
            ])
        );
    }
}
