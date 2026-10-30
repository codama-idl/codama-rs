use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{ConstantValueNode, HasTransforms, SentinelTransformNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

/// Parses `sentinel(type, constant(...))` into the type with a trailing `sentinelTransformNode`.
pub(crate) fn sentinel_type_node_from_meta(meta: &Meta) -> syn::Result<TypeNode> {
    let pl = meta.assert_directive("sentinel")?.as_path_list()?;
    let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
    let mut sentinel: SetOnce<ConstantValueNode> = SetOnce::new("sentinel");

    pl.each(|ref meta| match meta.path_str().as_str() {
        "type" => r#type.set(TypeNode::from_meta(meta.as_value()?)?, meta),
        "sentinel" => sentinel.set(ConstantValueNode::from_meta(meta.as_value()?)?, meta),
        "constant" => sentinel.set(ConstantValueNode::from_meta(meta)?, meta),
        _ => {
            if meta.is_path_or_list() {
                return r#type.set(TypeNode::from_meta(meta)?, meta);
            }
            Err(meta.error("unrecognized attribute"))
        }
    })?;

    let r#type = r#type.take(meta)?;
    Ok(r#type.add_transform(SentinelTransformNode::new(sentinel.take(meta)?)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{IntegerFormat::U8, IntegerTypeNode, IntegerValueNode, StringTypeNode};

    #[test]
    fn parses() {
        for meta in [
            syn::parse_quote! { sentinel(string, constant(integer(u8), 255)) },
            syn::parse_quote! { sentinel(type = string, sentinel = constant(integer(u8), 255)) },
        ] {
            assert_eq!(
                sentinel_type_node_from_meta(&meta).unwrap(),
                StringTypeNode::utf8()
                    .add_transform(SentinelTransformNode::new(ConstantValueNode::new(
                        IntegerTypeNode::le(U8),
                        IntegerValueNode::new(255u64)
                    )))
                    .into()
            );
        }
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { sentinel(string) },
                "sentinel is missing",
            ),
            (
                syn::parse_quote! { sentinel(constant(integer(u8), 255)) },
                "type is missing",
            ),
            (
                syn::parse_quote! { sentinel(foo = 42) },
                "unrecognized attribute",
            ),
        ] {
            let error = sentinel_type_node_from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
