use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{EnumTypeNode, EnumVariantTypeNode, IntegerTypeNode, TypeNode, TypeNodeTrait};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for EnumTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("enum")?;
        let mut variants: Vec<EnumVariantTypeNode> = Vec::new();
        let mut size: SetOnce<IntegerTypeNode> = SetOnce::new("size");

        if !meta.is_path_or_empty_list() {
            meta.as_path_list()?
                .each(|ref meta| match meta.path_str().as_str() {
                    "size" => {
                        let value = meta.as_value()?;
                        let node = TypeNode::from_meta(value)?;
                        let size_node = match IntegerTypeNode::try_from_type_node(node) {
                            Ok(node) => node,
                            Err(_) => return Err(value.error("size must be an IntegerTypeNode")),
                        };
                        size.set(size_node, meta)
                    }
                    "variant" => {
                        variants.push(EnumVariantTypeNode::from_meta(meta)?);
                        Ok(())
                    }
                    _ => Err(meta.error("expected variant(...) or size = ...")),
                })?;
        }

        let mut node = EnumTypeNode::new(variants);
        if let Some(size) = size.option() {
            node.size = size;
        }
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{
        IntegerFormat::{U16, U8},
        TupleTypeNode,
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
                EnumVariantTypeNode::new("pending"),
                EnumVariantTypeNode::new("done"),
            ])
            .into()
        );
    }

    #[test]
    fn tuple_variant() {
        assert_type!(
            { enum(variant("pair", tuple(integer(u8)))) },
            EnumTypeNode::new(vec![EnumVariantTypeNode::with_data(
                "pair",
                TupleTypeNode::new(vec![IntegerTypeNode::le(U8).into()])
            )])
            .into()
        );
    }

    #[test]
    fn size_defaults_to_u8() {
        assert_type!(
            { enum(variant("a")) },
            EnumTypeNode {
                size: IntegerTypeNode::le(U8),
                ..EnumTypeNode::new(vec![EnumVariantTypeNode::new("a")])
            }
            .into()
        );
    }

    #[test]
    fn explicit_size() {
        assert_type!(
            { enum(variant("a"), size = integer(u16)) },
            EnumTypeNode {
                size: IntegerTypeNode::le(U16),
                ..EnumTypeNode::new(vec![EnumVariantTypeNode::new("a")])
            }
            .into()
        );
    }

    #[test]
    fn invalid_size() {
        assert_type_err!(
            { enum(size = string) },
            "size must be an IntegerTypeNode"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ enum(foo = 42) }, "expected variant(...) or size = ...");
    }
}
