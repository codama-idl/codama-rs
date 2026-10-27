use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{ConstantValueNode, HasTransforms, HiddenSuffixTransformNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

/// Parses `hidden_suffix(type, constant(...), ...)` into the type with a trailing `hiddenSuffixTransformNode`.
pub(crate) fn hidden_suffix_type_node_from_meta(meta: &Meta) -> syn::Result<TypeNode> {
    let pl = meta.assert_directive("hidden_suffix")?.as_path_list()?;
    let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
    let mut suffix: Vec<ConstantValueNode> = Vec::new();

    pl.each(|ref meta| match meta.path_str().as_str() {
        "type" => r#type.set(TypeNode::from_meta(meta.as_value()?)?, meta),
        // Every `constant(...)` is appended, so the suffix keeps its order.
        "constant" => {
            suffix.push(ConstantValueNode::from_meta(meta)?);
            Ok(())
        }
        _ => {
            if meta.is_path_or_list() {
                return r#type.set(TypeNode::from_meta(meta)?, meta);
            }
            Err(meta.error("unrecognized attribute"))
        }
    })?;

    let r#type = r#type.take(meta)?;
    Ok(r#type.add_transform(HiddenSuffixTransformNode::new(suffix)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{IntegerFormat::U8, IntegerTypeNode, IntegerValueNode, StringTypeNode};

    fn constant(value: u64) -> ConstantValueNode {
        ConstantValueNode::new(IntegerTypeNode::le(U8), IntegerValueNode::new(value))
    }

    #[test]
    fn parses() {
        for (meta, expected) in [
            (
                syn::parse_quote! { hidden_suffix(string, constant(integer(u8), 1)) },
                vec![constant(1)],
            ),
            (
                syn::parse_quote! { hidden_suffix(string, constant(integer(u8), 1), constant(integer(u8), 2)) },
                vec![constant(1), constant(2)],
            ),
            (syn::parse_quote! { hidden_suffix(string) }, vec![]),
        ] {
            assert_eq!(
                hidden_suffix_type_node_from_meta(&meta).unwrap(),
                StringTypeNode::utf8()
                    .add_transform(HiddenSuffixTransformNode::new(expected))
                    .into()
            );
        }
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { hidden_suffix(constant(integer(u8), 1)) },
                "type is missing",
            ),
            (
                syn::parse_quote! { hidden_suffix(foo = 42) },
                "unrecognized attribute",
            ),
        ] {
            let error = hidden_suffix_type_node_from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
