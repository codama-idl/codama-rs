use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{CountNode, SetTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for SetTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("set")?.as_path_list()?;
        let mut item: SetOnce<TypeNode> = SetOnce::new("item");
        let mut count: SetOnce<CountNode> = SetOnce::new("count");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "item" => item.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            "count" => count.set(CountNode::from_meta(meta.as_value()?)?, meta),
            "fixed_count" | "prefixed_count" | "remainder_count" => {
                count.set(CountNode::from_meta(meta)?, meta)
            }
            _ => {
                if meta.is_path_or_list() {
                    return item.set(TypeNode::from_meta(meta)?, meta);
                }
                if meta.as_expr().is_ok() {
                    return count.set(CountNode::from_meta(meta)?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        Ok(SetTypeNode::new(item.take(meta)?, count.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{
        FixedCountNode,
        NumberFormat::{U32, U8},
        NumberTypeNode, PrefixedCountNode,
    };

    #[test]
    fn implicit() {
        assert_type!(
            { set(number(u8), fixed_count(3)) },
            SetTypeNode::new(NumberTypeNode::le(U8), FixedCountNode::new(3)).into()
        );
    }

    #[test]
    fn bare_integer_count() {
        assert_type!(
            { set(number(u8), 3) },
            SetTypeNode::new(NumberTypeNode::le(U8), FixedCountNode::new(3)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { set(item = number(u8), count = prefixed_count(number(u32))) },
            SetTypeNode::new(
                NumberTypeNode::le(U8),
                PrefixedCountNode::new(NumberTypeNode::le(U32))
            )
            .into()
        );
    }

    #[test]
    fn count_missing() {
        assert_type_err!({ set(number(u8)) }, "count is missing");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ set(foo = 42) }, "unrecognized attribute");
    }
}
