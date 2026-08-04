use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{CountNode, MapTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for MapTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("map")?.as_path_list()?;
        let mut key: SetOnce<TypeNode> = SetOnce::new("key");
        let mut value: SetOnce<TypeNode> = SetOnce::new("value");
        let mut count: SetOnce<CountNode> = SetOnce::new("count");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "key" => key.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            "value" => value.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            "count" => count.set(CountNode::from_meta(meta.as_value()?)?, meta),
            "fixed_count" | "prefixed_count" | "remainder_count" => {
                count.set(CountNode::from_meta(meta)?, meta)
            }
            _ => {
                // Key and value are both type nodes, so positional args fill
                // `key` then `value` and are never swapped.
                if meta.is_path_or_list() {
                    return match key.is_set() {
                        false => key.set(TypeNode::from_meta(meta)?, meta),
                        true => value.set(TypeNode::from_meta(meta)?, meta),
                    };
                }
                if meta.as_expr().is_ok() {
                    return count.set(CountNode::from_meta(meta)?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        Ok(MapTypeNode::new(
            key.take(meta)?,
            value.take(meta)?,
            count.take(meta)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{
        FixedCountNode,
        NumberFormat::{U32, U8},
        NumberTypeNode, PrefixedCountNode, PublicKeyTypeNode, StringTypeNode,
    };

    #[test]
    fn implicit() {
        assert_type!(
            { map(public_key, number(u8), fixed_count(3)) },
            MapTypeNode::new(
                PublicKeyTypeNode::new(),
                NumberTypeNode::le(U8),
                FixedCountNode::new(3)
            )
            .into()
        );
    }

    #[test]
    fn bare_integer_count() {
        assert_type!(
            { map(string, number(u8), 3) },
            MapTypeNode::new(
                StringTypeNode::utf8(),
                NumberTypeNode::le(U8),
                FixedCountNode::new(3)
            )
            .into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            {
                map(
                    key = string,
                    value = number(u8),
                    count = prefixed_count(number(u32)),
                )
            },
            MapTypeNode::new(
                StringTypeNode::utf8(),
                NumberTypeNode::le(U8),
                PrefixedCountNode::new(NumberTypeNode::le(U32))
            )
            .into()
        );
    }

    #[test]
    fn positional_order() {
        // Positional args fill `key` then `value` and are never swapped.
        assert_type!(
            { map(string, public_key, 1) },
            MapTypeNode::new(
                StringTypeNode::utf8(),
                PublicKeyTypeNode::new(),
                FixedCountNode::new(1)
            )
            .into()
        );
    }

    #[test]
    fn value_missing() {
        assert_type_err!({ map(string, fixed_count(3)) }, "value is missing");
    }

    #[test]
    fn count_missing() {
        assert_type_err!({ map(string, number(u8)) }, "count is missing");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ map(foo = 42) }, "unrecognized attribute");
    }
}
