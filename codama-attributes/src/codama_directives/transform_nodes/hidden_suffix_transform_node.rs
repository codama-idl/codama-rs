use super::parse_hidden_transform;
use codama_nodes::{HasTransforms, HiddenSuffixTransformNode, TypeNode};
use codama_syn_helpers::Meta;

/// Parses `hidden_suffix(type, constant(...), ...)` into the type with a trailing `hiddenSuffixTransformNode`.
pub(crate) fn hidden_suffix_from_meta(meta: &Meta) -> syn::Result<TypeNode> {
    let (r#type, suffix) = parse_hidden_transform(meta, "hidden_suffix")?;
    Ok(r#type.add_transform(HiddenSuffixTransformNode::new(suffix)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        ConstantValueNode, IntegerFormat::U8, IntegerTypeNode, IntegerValueNode, StringTypeNode,
    };

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
                hidden_suffix_from_meta(&meta).unwrap(),
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
            let error = hidden_suffix_from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
