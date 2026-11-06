use crate::{EnumTypeNode, EnumVariantTypeNode, IntegerTypeNode, U8};

impl EnumTypeNode {
    pub fn new(variants: Vec<EnumVariantTypeNode>) -> Self {
        Self {
            variants,
            size: IntegerTypeNode::le(U8),
            transforms: vec![],
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, U32};

    #[test]
    fn new() {
        let r#enum = EnumTypeNode::new(vec![
            EnumVariantTypeNode::new("quit"),
            EnumVariantTypeNode::with_data(
                "move",
                TupleTypeNode::new(vec![
                    IntegerTypeNode::le(U8).into(),
                    IntegerTypeNode::le(U8).into(),
                ]),
            ),
            EnumVariantTypeNode::with_data(
                "write",
                StructTypeNode::new(vec![StructFieldTypeNode::new(
                    "message",
                    StringTypeNode::utf8(),
                )]),
            ),
        ]);
        assert_eq!(r#enum.variants.len(), 3);
        assert_eq!(r#enum.size, IntegerTypeNode::le(U8));
    }

    #[test]
    fn direct_instantiation() {
        let r#enum = EnumTypeNode {
            variants: vec![],
            size: IntegerTypeNode::le(U32),
            transforms: vec![],
            plugins: vec![],
        };
        assert_eq!(r#enum.size, IntegerTypeNode::le(U32));
    }

    #[test]
    fn to_json() {
        let node = EnumTypeNode::new(vec![EnumVariantTypeNode::new("myVariant")]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumTypeNode","variants":[{"kind":"enumVariantTypeNode","identifier":"myVariant"}],"size":{"kind":"integerTypeNode","format":"u8","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"enumTypeNode","variants":[{"kind":"enumVariantTypeNode","identifier":"myVariant"}],"size":{"kind":"integerTypeNode","format":"u8","endian":"le"}}"#;
        let node: EnumTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            EnumTypeNode::new(vec![EnumVariantTypeNode::new("myVariant")])
        );
    }
}
