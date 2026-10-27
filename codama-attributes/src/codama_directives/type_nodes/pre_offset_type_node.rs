use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{HasTransforms, PreOffsetStrategy, PreOffsetTransformNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

/// Parses `pre_offset(type, offset, strategy)` into the type with a trailing `preOffsetTransformNode`.
pub(crate) fn pre_offset_type_node_from_meta(meta: &Meta) -> syn::Result<TypeNode> {
    let pl = meta.assert_directive("pre_offset")?.as_path_list()?;
    let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
    let mut offset: SetOnce<i64> = SetOnce::new("offset");
    let mut strategy: SetOnce<PreOffsetStrategy> = SetOnce::new("strategy");

    pl.each(|ref meta| {
        // Strategies are written as bare paths, so they are claimed before a
        // bare path could be read as a positional type node.
        if let Some(value) = meta.as_path().ok().and_then(parse_strategy) {
            return strategy.set(value, meta);
        }
        match meta.path_str().as_str() {
            "type" => r#type.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            "offset" => offset.set(meta.as_value()?.as_expr()?.as_signed_integer()?, meta),
            "strategy" => {
                let path = meta.as_value()?.as_expr()?.as_path()?;
                match parse_strategy(path) {
                    Some(value) => strategy.set(value, meta),
                    None => Err(path.error("invalid strategy")),
                }
            }
            _ => {
                if meta.is_path_or_list() {
                    return r#type.set(TypeNode::from_meta(meta)?, meta);
                }
                if let Ok(expr) = meta.as_expr() {
                    return offset.set(expr.as_signed_integer()?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        }
    })?;

    let r#type = r#type.take(meta)?;
    let transform = PreOffsetTransformNode::new(offset.take(meta)?, strategy.take(meta)?);
    Ok(r#type.add_transform(transform))
}

fn parse_strategy(path: &syn::Path) -> Option<PreOffsetStrategy> {
    match path.to_string().as_str() {
        "absolute" => Some(PreOffsetStrategy::Absolute),
        "padded" => Some(PreOffsetStrategy::Padded),
        "relative" => Some(PreOffsetStrategy::Relative),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::StringTypeNode;

    #[test]
    fn parses() {
        for (meta, expected) in [
            (
                syn::parse_quote! { pre_offset(string, 4, relative) },
                PreOffsetTransformNode::relative(4),
            ),
            (
                syn::parse_quote! { pre_offset(type = string, offset = 4, strategy = absolute) },
                PreOffsetTransformNode::absolute(4),
            ),
            (
                syn::parse_quote! { pre_offset(string, 8, padded) },
                PreOffsetTransformNode::padded(8),
            ),
            (
                syn::parse_quote! { pre_offset(string, -4, relative) },
                PreOffsetTransformNode::relative(-4),
            ),
        ] {
            assert_eq!(
                pre_offset_type_node_from_meta(&meta).unwrap(),
                StringTypeNode::utf8().add_transform(expected).into()
            );
        }
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { pre_offset(string, 4) },
                "strategy is missing",
            ),
            (
                syn::parse_quote! { pre_offset(string, relative) },
                "offset is missing",
            ),
            (
                syn::parse_quote! { pre_offset(string, 4, strategy = nonsense) },
                "invalid strategy",
            ),
            (
                syn::parse_quote! { pre_offset(string, 4, nonsense) },
                "unrecognized type",
            ),
            (
                syn::parse_quote! { pre_offset(foo = 42) },
                "unrecognized attribute",
            ),
        ] {
            let error = pre_offset_type_node_from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
