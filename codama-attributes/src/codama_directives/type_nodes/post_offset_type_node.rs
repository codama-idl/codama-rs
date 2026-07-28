use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{PostOffsetStrategy, PostOffsetTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for PostOffsetTypeNode<TypeNode> {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("post_offset")?.as_path_list()?;
        let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
        let mut offset: SetOnce<i32> = SetOnce::new("offset");
        let mut strategy: SetOnce<PostOffsetStrategy> = SetOnce::new("strategy");

        pl.each(|ref meta| {
            // Strategies are written as bare paths. This also keeps the
            // `pre_offset` strategy distinct from the `pre_offset(...)` type node,
            // which is a list rather than a bare path.
            if let Some(value) = bare_path(meta).as_deref().and_then(parse_strategy) {
                return strategy.set(value, meta);
            }
            match meta.path_str().as_str() {
                "type" => r#type.set(TypeNode::from_meta(meta.as_value()?)?, meta),
                "offset" => offset.set(meta.as_value()?.as_expr()?.as_signed_integer()?, meta),
                "strategy" => {
                    let path = meta.as_value()?.as_expr()?.as_path()?;
                    match parse_strategy(&path.to_string()) {
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

        Ok(PostOffsetTypeNode::new(
            r#type.take(meta)?,
            strategy.take(meta)?,
            offset.take(meta)?,
        ))
    }
}

fn bare_path(meta: &Meta) -> Option<String> {
    meta.as_expr()
        .ok()?
        .as_path()
        .ok()
        .map(|path| path.to_string())
}

fn parse_strategy(name: &str) -> Option<PostOffsetStrategy> {
    match name {
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
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{PreOffsetTypeNode, StringTypeNode};

    #[test]
    fn implicit() {
        assert_type!(
            { post_offset(string, 4, relative) },
            PostOffsetTypeNode::relative(StringTypeNode::utf8(), 4).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { post_offset(type = string, offset = 4, strategy = absolute) },
            PostOffsetTypeNode::absolute(StringTypeNode::utf8(), 4).into()
        );
    }

    #[test]
    fn pre_offset_strategy() {
        assert_type!(
            { post_offset(string, 0, pre_offset) },
            PostOffsetTypeNode::pre_offset(StringTypeNode::utf8(), 0).into()
        );
    }

    #[test]
    fn wrapping_a_pre_offset_type_node() {
        // A bare `pre_offset` is the strategy, `pre_offset(...)` is the type node.
        assert_type!(
            { post_offset(pre_offset(string, 4, relative), 0, pre_offset) },
            PostOffsetTypeNode::pre_offset(
                PreOffsetTypeNode::relative(StringTypeNode::utf8(), 4),
                0
            )
            .into()
        );
    }

    #[test]
    fn strategy_missing() {
        assert_type_err!({ post_offset(string, 4) }, "strategy is missing");
    }

    #[test]
    fn invalid_strategy() {
        assert_type_err!(
            { post_offset(string, 4, strategy = nonsense) },
            "invalid strategy"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ post_offset(foo = 42) }, "unrecognized attribute");
    }
}
