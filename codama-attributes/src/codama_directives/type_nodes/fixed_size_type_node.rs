use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{FixedSizeTransformNode, HasTransforms, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

/// Parses `fixed_size(type, size)` into the type with a trailing `fixedSizeTransformNode`.
pub(crate) fn fixed_size_type_node_from_meta(meta: &Meta) -> syn::Result<TypeNode> {
    let pl = meta.assert_directive("fixed_size")?.as_path_list()?;
    let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
    let mut size: SetOnce<u64> = SetOnce::new("size");

    pl.each(|ref meta| match meta.path_str().as_str() {
        "type" => {
            let node = TypeNode::from_meta(meta.as_value()?)?;
            r#type.set(node, meta)
        }
        "size" => size.set(meta.as_value()?.as_expr()?.as_unsigned_integer()?, meta),
        _ => {
            if meta.is_path_or_list() {
                return r#type.set(TypeNode::from_meta(meta)?, meta);
            }
            if let Ok(expr) = meta.as_expr() {
                return size.set(expr.as_unsigned_integer()?, meta);
            }
            Err(meta.error("unrecognized attribute"))
        }
    })?;

    let r#type = r#type.take(meta)?;
    Ok(r#type.add_transform(FixedSizeTransformNode::new(size.take(meta)?)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_type;
    use codama_nodes::{
        BooleanTypeNode, DefinedTypeLinkNode, IntegerFormat::U32, IntegerTypeNode,
        SizePrefixTransformNode, StringTypeNode,
    };

    #[test]
    fn parses() {
        for meta in [
            syn::parse_quote! { fixed_size(type = boolean, size = 42) },
            syn::parse_quote! { fixed_size(size = 42, type = boolean) },
            syn::parse_quote! { fixed_size(boolean, 42) },
            syn::parse_quote! { fixed_size(42, boolean) },
        ] {
            assert_eq!(
                fixed_size_type_node_from_meta(&meta).unwrap(),
                BooleanTypeNode::default()
                    .add_transform(FixedSizeTransformNode::new(42))
                    .into()
            );
        }
    }

    #[test]
    fn nested_wrappers_list_innermost_first() {
        assert_type!(
            { fixed_size(size_prefix(string, integer(u32)), 10) },
            StringTypeNode::utf8()
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32)))
                .add_transform(FixedSizeTransformNode::new(10))
                .into()
        );
    }

    #[test]
    fn link() {
        assert_type!(
            { fixed_size(link("foo"), 8) },
            DefinedTypeLinkNode::new("foo")
                .add_transform(FixedSizeTransformNode::new(8))
                .into()
        );
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { fixed_size(unrecognized) },
                "unrecognized type",
            ),
            (
                syn::parse_quote! { fixed_size(foo = 42) },
                "unrecognized attribute",
            ),
        ] {
            let error = fixed_size_type_node_from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
