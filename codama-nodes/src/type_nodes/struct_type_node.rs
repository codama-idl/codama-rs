use crate::{StructFieldTypeNode, StructTypeNode};

impl StructTypeNode {
    pub fn new(fields: Vec<StructFieldTypeNode>) -> Self {
        Self {
            fields,
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
        let node = StructTypeNode::new(vec![
            StructFieldTypeNode::new("age", IntegerTypeNode::le(U32)),
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
        ]);
        assert_eq!(
            node.fields,
            vec![
                StructFieldTypeNode::new("age", IntegerTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            ]
        );
    }

    #[test]
    fn to_json() {
        let node = StructTypeNode::new(vec![
            StructFieldTypeNode::new("age", IntegerTypeNode::le(U32)),
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
        ]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","identifier":"age","type":{"kind":"integerTypeNode","format":"u32","endian":"le"}},{"kind":"structFieldTypeNode","identifier":"name","type":{"kind":"stringTypeNode","encoding":"utf8"}}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","identifier":"age","type":{"kind":"integerTypeNode","format":"u32","endian":"le"}},{"kind":"structFieldTypeNode","identifier":"name","type":{"kind":"stringTypeNode","encoding":"utf8"}}]}"#;
        let node: StructTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("age", IntegerTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            ])
        );
    }
}
