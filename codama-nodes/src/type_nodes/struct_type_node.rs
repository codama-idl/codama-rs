use crate::StructFieldTypeNode;
use codama_nodes_derive::type_node;

#[type_node]
#[derive(Default)]
pub struct StructTypeNode {
    // Children.
    pub fields: Vec<StructFieldTypeNode>,
}

impl Into<crate::Node> for StructTypeNode {
    fn into(self) -> crate::Node {
        crate::Node::Type(self.into())
    }
}

impl StructTypeNode {
    pub fn new(fields: Vec<StructFieldTypeNode>) -> Self {
        Self { fields }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, StringTypeNode, U32};

    #[test]
    fn new() {
        let node = StructTypeNode::new(vec![
            StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
        ]);
        assert_eq!(
            node.fields,
            vec![
                StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            ]
        );
    }

    #[test]
    fn to_json() {
        let node = StructTypeNode::new(vec![
            StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
        ]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","name":"age","type":{"kind":"numberTypeNode","format":"u32","endian":"le"}},{"kind":"structFieldTypeNode","name":"name","type":{"kind":"stringTypeNode","encoding":"utf8"}}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","name":"age","type":{"kind":"numberTypeNode","format":"u32","endian":"le"}},{"kind":"structFieldTypeNode","name":"name","type":{"kind":"stringTypeNode","encoding":"utf8"}}]}"#;
        let node: StructTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            ])
        );
    }
}
