use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{
    HasTransforms, IntegerTypeNode, SizePrefixTransformNode, TypeNode, TypeNodeTrait,
};
use codama_syn_helpers::{extensions::*, Meta};

/// Parses `size_prefix(type, prefix)` into the type with a trailing `sizePrefixTransformNode`.
pub(crate) fn size_prefix_type_node_from_meta(meta: &Meta) -> syn::Result<TypeNode> {
    let pl = meta.assert_directive("size_prefix")?.as_path_list()?;
    let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
    let mut prefix: SetOnce<IntegerTypeNode> = SetOnce::new("prefix");

    pl.each(|ref meta| match meta.path_str().as_str() {
        "type" => r#type.set(TypeNode::from_meta(meta.as_value()?)?, meta),
        "prefix" => prefix.set(parse_prefix(meta.as_value()?)?, meta),
        _ => {
            // Both are type nodes, so positional args fill `type` then `prefix`.
            if meta.is_path_or_list() {
                return match r#type.is_set() {
                    false => r#type.set(TypeNode::from_meta(meta)?, meta),
                    true => prefix.set(parse_prefix(meta)?, meta),
                };
            }
            Err(meta.error("unrecognized attribute"))
        }
    })?;

    let r#type = r#type.take(meta)?;
    Ok(r#type.add_transform(SizePrefixTransformNode::new(prefix.take(meta)?)))
}

fn parse_prefix(meta: &Meta) -> syn::Result<IntegerTypeNode> {
    IntegerTypeNode::try_from_type_node(TypeNode::from_meta(meta)?)
        .map_err(|_| meta.error("prefix must be an IntegerTypeNode"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_type;
    use codama_nodes::{
        BytesTypeNode, FixedSizeTransformNode,
        IntegerFormat::{U16, U32, U8},
        OptionTypeNode, StringTypeNode,
    };

    #[test]
    fn parsed_fields() {
        for (meta, expected) in [
            (
                syn::parse_quote! { size_prefix(string, integer(u32)) },
                TypeNode::from(
                    StringTypeNode::utf8()
                        .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32))),
                ),
            ),
            (
                syn::parse_quote! { size_prefix(bytes, integer(u16, be)) },
                BytesTypeNode::new()
                    .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::be(U16)))
                    .into(),
            ),
        ] {
            assert_eq!(size_prefix_type_node_from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn equivalent_syntaxes() {
        for meta in [
            syn::parse_quote! { size_prefix(string, integer(u32)) },
            syn::parse_quote! { size_prefix(string, prefix = integer(u32)) },
            syn::parse_quote! { size_prefix(type = string, prefix = integer(u32)) },
            syn::parse_quote! { size_prefix(prefix = integer(u32), type = string) },
            syn::parse_quote! { size_prefix(prefix = integer(u32), string) },
        ] {
            assert_eq!(
                size_prefix_type_node_from_meta(&meta).unwrap(),
                StringTypeNode::utf8()
                    .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32)))
                    .into()
            );
        }
    }

    #[test]
    fn nested() {
        assert_type!(
            { option(size_prefix(string, integer(u8))) },
            OptionTypeNode::new(
                StringTypeNode::utf8()
                    .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U8)))
            )
            .into()
        );

        let meta: Meta = syn::parse_quote! { option(size_prefix(string, integer(u8))) };
        let node = OptionTypeNode::from_meta(&meta).unwrap();
        assert_eq!(
            *node.item,
            TypeNode::String(
                StringTypeNode::utf8()
                    .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U8)))
            )
        );
    }

    #[test]
    fn nested_prefix() {
        assert_type!(
            { size_prefix(string, fixed_size(integer(u32), 4)) },
            StringTypeNode::utf8()
                .add_transform(SizePrefixTransformNode::new(
                    IntegerTypeNode::le(U32).add_transform(FixedSizeTransformNode::new(4))
                ))
                .into()
        );
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { size_prefix(string, public_key) },
                "prefix must be an IntegerTypeNode",
            ),
            (
                syn::parse_quote! { size_prefix(type = string, prefix = public_key) },
                "prefix must be an IntegerTypeNode",
            ),
            (
                syn::parse_quote! { size_prefix(string, float(f32)) },
                "prefix must be an IntegerTypeNode",
            ),
            (
                syn::parse_quote! { size_prefix(integer(u32), string) },
                "prefix must be an IntegerTypeNode",
            ),
            (
                syn::parse_quote! { size_prefix(prefix = integer(u32)) },
                "type is missing",
            ),
            (
                syn::parse_quote! { size_prefix(string) },
                "prefix is missing",
            ),
            (
                syn::parse_quote! { size_prefix(type = string, type = bytes) },
                "type is already set",
            ),
            (
                syn::parse_quote! { size_prefix(string, integer(u32), integer(u8)) },
                "prefix is already set",
            ),
            (
                syn::parse_quote! { size_prefix(unrecognized, integer(u32)) },
                "unrecognized type",
            ),
            (
                syn::parse_quote! { size_prefix(string, foo = 42) },
                "unrecognized attribute",
            ),
        ] {
            let error = size_prefix_type_node_from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
