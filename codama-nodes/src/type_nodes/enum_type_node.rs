use crate::{EnumVariantTypeNode, NestedTypeNode, NumberTypeNode, U8};
use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq, Clone)]
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
}
