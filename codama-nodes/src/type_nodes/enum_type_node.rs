use crate::{EnumVariantTypeNode, NestedTypeNode, NumberTypeNode, U8};
use codama_nodes_derive::type_node;

#[type_node]
pub struct EnumTypeNode {
    // Children.
    pub variants: Vec<EnumVariantTypeNode>,
    pub size: NestedTypeNode<NumberTypeNode>,
}

impl EnumTypeNode {
    pub fn new(variants: Vec<EnumVariantTypeNode>) -> Self {
        Self {
            variants,
            size: NestedTypeNode::Value(NumberTypeNode::le(U8)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode,
        NumberTypeNode, StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, U32,
    };

    use super::*;

    #[test]
    fn new() {
        let r#enum = EnumTypeNode::new(vec![
            EnumEmptyVariantTypeNode::new("quit", None).into(),
            EnumTupleVariantTypeNode::new(
                "move",
                TupleTypeNode::new(vec![
                    NumberTypeNode::le(U8).into(),
                    NumberTypeNode::le(U8).into(),
                ]),
                None,
            )
            .into(),
            EnumStructVariantTypeNode::new(
                "write",
                StructTypeNode::new(vec![StructFieldTypeNode::new(
                    "message",
                    StringTypeNode::utf8(),
                )]),
                None,
            )
            .into(),
        ]);
        assert_eq!(r#enum.size, NestedTypeNode::Value(NumberTypeNode::le(U8)));
    }

    #[test]
    fn direct_instantiation() {
        let r#enum = EnumTypeNode {
            variants: vec![],
            size: NumberTypeNode::le(U32).into(),
        };
        assert_eq!(r#enum.size, NestedTypeNode::Value(NumberTypeNode::le(U32)));
    }

    #[test]
    fn to_json() {
        let node = EnumTypeNode::new(vec![EnumEmptyVariantTypeNode::new("myVariant", None).into()]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumTypeNode","variants":[{"kind":"enumEmptyVariantTypeNode","name":"myVariant"}],"size":{"kind":"numberTypeNode","format":"u8","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"enumTypeNode","variants":[{"kind":"enumEmptyVariantTypeNode","name":"myVariant"}],"size":{"kind":"numberTypeNode","format":"u8","endian":"le"}}"#;
        let node: EnumTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            EnumTypeNode::new(vec![EnumEmptyVariantTypeNode::new("myVariant", None).into()])
        );
    }
}
