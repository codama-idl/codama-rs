use super::parse_offset_transform;
use codama_nodes::{HasTransforms, PostOffsetStrategy, PostOffsetTransformNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

/// Parses `post_offset(type, offset, strategy)` into the type with a trailing `postOffsetTransformNode`.
pub(crate) fn post_offset_from_meta(meta: &Meta) -> syn::Result<TypeNode> {
    let (r#type, offset, strategy) = parse_offset_transform(meta, "post_offset", parse_strategy)?;
    Ok(r#type.add_transform(PostOffsetTransformNode::new(offset, strategy)))
}

fn parse_strategy(path: &syn::Path) -> Option<PostOffsetStrategy> {
    match path.to_string().as_str() {
        "absolute" => Some(PostOffsetStrategy::Absolute),
        "padded" => Some(PostOffsetStrategy::Padded),
        "pre_offset" => Some(PostOffsetStrategy::PreOffset),
        "relative" => Some(PostOffsetStrategy::Relative),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{PreOffsetTransformNode, StringTypeNode};

    #[test]
    fn parses() {
        for (meta, expected) in [
            (
                syn::parse_quote! { post_offset(string, 4, relative) },
                PostOffsetTransformNode::relative(4),
            ),
            (
                syn::parse_quote! { post_offset(type = string, offset = 4, strategy = absolute) },
                PostOffsetTransformNode::absolute(4),
            ),
            (
                syn::parse_quote! { post_offset(string, 0, pre_offset) },
                PostOffsetTransformNode::pre_offset(0),
            ),
            (
                syn::parse_quote! { post_offset(string, -4, relative) },
                PostOffsetTransformNode::relative(-4),
            ),
            (
                syn::parse_quote! { post_offset(string, -9223372036854775808, relative) },
                PostOffsetTransformNode::relative(i64::MIN),
            ),
        ] {
            assert_eq!(
                post_offset_from_meta(&meta).unwrap(),
                StringTypeNode::utf8().add_transform(expected).into()
            );
        }
    }

    #[test]
    fn wraps_a_pre_offset() {
        let meta =
            syn::parse_quote! { post_offset(pre_offset(string, 4, relative), 0, pre_offset) };
        assert_eq!(
            post_offset_from_meta(&meta).unwrap(),
            StringTypeNode::utf8()
                .add_transform(PreOffsetTransformNode::relative(4))
                .add_transform(PostOffsetTransformNode::pre_offset(0))
                .into()
        );
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { post_offset(string, 4) },
                "strategy is missing",
            ),
            (
                syn::parse_quote! { post_offset(string, 4, strategy = nonsense) },
                "invalid strategy",
            ),
            (
                syn::parse_quote! { post_offset(string, 4, nonsense) },
                "unrecognized type",
            ),
            (
                syn::parse_quote! { post_offset(foo = 42) },
                "unrecognized attribute",
            ),
        ] {
            let error = post_offset_from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
