use codama_nodes_derive::{Node, TypeNode};

use super::{EnumVariantTypeNode, NestedTypeNode, NumberTypeNode, U8};

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct EnumTypeNode {
    // Children.
    pub variants: Vec<EnumVariantTypeNode>,
    pub size: NestedTypeNode<NumberTypeNode>,
}

impl EnumTypeNode {
    pub fn new(variants: Vec<EnumVariantTypeNode>) -> Self {
        Self {
            variants: variants.into(),
            size: NestedTypeNode::Value(NumberTypeNode::le(U8)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode,
        NumberTypeNode, StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode,
    };

    use super::*;

    #[test]
    fn new() {
        let r#enum = EnumTypeNode::new(vec![
            EnumEmptyVariantTypeNode::new("quit", None).into(), // TODO: Try to remove the `.into()`.
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
}
