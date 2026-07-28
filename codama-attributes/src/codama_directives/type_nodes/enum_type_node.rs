use crate::codama_directives::nested_number::nested_number;
use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{EnumTypeNode, EnumVariantTypeNode, NestedTypeNode, NumberTypeNode, U8};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for EnumTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("enum")?;
        let mut variants: Vec<EnumVariantTypeNode> = Vec::new();
        let mut size: SetOnce<NestedTypeNode<NumberTypeNode>> = SetOnce::new("size");

        if !meta.is_path_or_empty_list() {
            meta.as_path_list()?
                .each(|ref meta| match meta.path_str().as_str() {
                    "size" => size.set(nested_number(meta.as_value()?, "size")?, meta),
                    "variant" => {
                        variants.push(EnumVariantTypeNode::from_meta(meta)?);
                        Ok(())
                    }
                    _ => Err(meta.error("expected variant(...) or size = ...")),
                })?;
        }

        Ok(EnumTypeNode {
            variants,
            size: size
                .option()
                .unwrap_or(NestedTypeNode::Value(NumberTypeNode::le(U8))),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{
        EnumEmptyVariantTypeNode, EnumTupleVariantTypeNode, NumberFormat::U16, TupleTypeNode,
    };

    #[test]
    fn empty() {
        assert_type!({ enum }, EnumTypeNode::new(vec![]).into());
        assert_type!({ enum() }, EnumTypeNode::new(vec![]).into());
    }

    #[test]
    fn empty_variants() {
        assert_type!(
            { enum(variant("pending"), variant("done")) },
            EnumTypeNode::new(vec![
                EnumEmptyVariantTypeNode::new("pending").into(),
                EnumEmptyVariantTypeNode::new("done").into(),
            ])
            .into()
        );
    }

    #[test]
    fn tuple_variant() {
        assert_type!(
            { enum(variant("pair", tuple(number(u8)))) },
            EnumTypeNode::new(vec![EnumTupleVariantTypeNode::new(
                "pair",
                TupleTypeNode::new(vec![NumberTypeNode::le(U8).into()])
            )
            .into()])
            .into()
        );
    }

    #[test]
    fn size_defaults_to_u8() {
        assert_type!(
            { enum(variant("a")) },
            EnumTypeNode {
                variants: vec![EnumEmptyVariantTypeNode::new("a").into()],
                size: NestedTypeNode::Value(NumberTypeNode::le(U8)),
            }
            .into()
        );
    }

    #[test]
    fn explicit_size() {
        assert_type!(
            { enum(variant("a"), size = number(u16)) },
            EnumTypeNode {
                variants: vec![EnumEmptyVariantTypeNode::new("a").into()],
                size: NestedTypeNode::Value(NumberTypeNode::le(U16)),
            }
            .into()
        );
    }

    #[test]
    fn invalid_size() {
        assert_type_err!(
            { enum(size = string) },
            "size must be a NumberTypeNode"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ enum(foo = 42) }, "expected variant(...) or size = ...");
    }
}
