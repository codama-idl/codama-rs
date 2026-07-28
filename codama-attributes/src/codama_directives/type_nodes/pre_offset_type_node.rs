use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{PreOffsetStrategy, PreOffsetTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for PreOffsetTypeNode<TypeNode> {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("pre_offset")?.as_path_list()?;
        let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
        let mut offset: SetOnce<i32> = SetOnce::new("offset");
        let mut strategy: SetOnce<PreOffsetStrategy> = SetOnce::new("strategy");

        pl.each(|ref meta| {
            // Strategies are written as bare paths, so they are claimed before a
            // bare path could be read as a positional type node.
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

        Ok(PreOffsetTypeNode::new(
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

fn parse_strategy(name: &str) -> Option<PreOffsetStrategy> {
    match name {
        "absolute" => Some(PreOffsetStrategy::Absolute),
        "padded" => Some(PreOffsetStrategy::Padded),
        "relative" => Some(PreOffsetStrategy::Relative),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::StringTypeNode;

    #[test]
    fn implicit() {
        assert_type!(
            { pre_offset(string, 4, relative) },
            PreOffsetTypeNode::relative(StringTypeNode::utf8(), 4).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { pre_offset(type = string, offset = 4, strategy = absolute) },
            PreOffsetTypeNode::absolute(StringTypeNode::utf8(), 4).into()
        );
    }

    #[test]
    fn padded() {
        assert_type!(
            { pre_offset(string, 8, padded) },
            PreOffsetTypeNode::padded(StringTypeNode::utf8(), 8).into()
        );
    }

    #[test]
    fn negative_offset() {
        assert_type!(
            { pre_offset(string, -4, relative) },
            PreOffsetTypeNode::relative(StringTypeNode::utf8(), -4).into()
        );
    }

    #[test]
    fn strategy_missing() {
        assert_type_err!({ pre_offset(string, 4) }, "strategy is missing");
    }

    #[test]
    fn offset_missing() {
        assert_type_err!({ pre_offset(string, relative) }, "offset is missing");
    }

    #[test]
    fn invalid_strategy() {
        assert_type_err!(
            { pre_offset(string, 4, strategy = nonsense) },
            "invalid strategy"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ pre_offset(foo = 42) }, "unrecognized attribute");
    }
}
