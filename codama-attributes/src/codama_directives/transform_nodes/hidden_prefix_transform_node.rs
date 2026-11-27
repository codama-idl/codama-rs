use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{ConstantValueNode, HasTransforms, HiddenPrefixTransformNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

/// Parses `hidden_prefix(type, constant(...), ...)` into the type with a trailing `hiddenPrefixTransformNode`.
pub(crate) fn hidden_prefix_from_meta(meta: &Meta) -> syn::Result<TypeNode> {
    let (r#type, prefix) = parse_hidden_transform(meta, "hidden_prefix")?;
    Ok(r#type.add_transform(HiddenPrefixTransformNode::new(prefix)))
}

/// Parses a `hidden_prefix(type, constant(...), ...)` or `hidden_suffix(type, constant(...), ...)`
/// directive, named `directive`, into its type and its constants, in order.
pub(crate) fn parse_hidden_transform(
    meta: &Meta,
    directive: &str,
) -> syn::Result<(TypeNode, Vec<ConstantValueNode>)> {
    let pl = meta.assert_directive(directive)?.as_path_list()?;
    let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
    let mut constants: Vec<ConstantValueNode> = Vec::new();

    pl.each(|ref meta| match meta.path_str().as_str() {
        "type" => r#type.set(TypeNode::from_meta(meta.as_value()?)?, meta),
        // Every `constant(...)` is appended, so the constants keep their order.
        "constant" => {
            constants.push(ConstantValueNode::from_meta(meta)?);
            Ok(())
        }
        _ => {
            if meta.is_path_or_list() {
                return r#type.set(TypeNode::from_meta(meta)?, meta);
            }
            Err(meta.error("unrecognized attribute"))
        }
    })?;

    Ok((r#type.take(meta)?, constants))
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
                syn::parse_quote! { hidden_prefix(string, constant(integer(u8), 1)) },
                vec![constant(1)],
            ),
            (
                syn::parse_quote! { hidden_prefix(string, constant(integer(u8), 1), constant(integer(u8), 2)) },
                vec![constant(1), constant(2)],
            ),
            (syn::parse_quote! { hidden_prefix(string) }, vec![]),
        ] {
            assert_eq!(
                hidden_prefix_from_meta(&meta).unwrap(),
                StringTypeNode::utf8()
                    .add_transform(HiddenPrefixTransformNode::new(expected))
                    .into()
            );
        }
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { hidden_prefix(constant(integer(u8), 1)) },
                "type is missing",
            ),
            (
                syn::parse_quote! { hidden_prefix(foo = 42) },
                "unrecognized attribute",
            ),
        ] {
            let error = hidden_prefix_from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
