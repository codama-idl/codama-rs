use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{ArrayTypeNode, CountNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for ArrayTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("array")?.as_path_list()?;
        let mut item: SetOnce<TypeNode> = SetOnce::new("item");
        let mut count: SetOnce<CountNode> = SetOnce::new("count");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "item" => item.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            "count" => count.set(CountNode::from_meta(meta.as_value()?)?, meta),
            // The count nodes have their own directives, so they are recognised
            // by name before falling through to the item.
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

        Ok(ArrayTypeNode::new(item.take(meta)?, count.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{
        FixedCountNode,
        NumberFormat::{U32, U8},
        NumberTypeNode, PrefixedCountNode, RemainderCountNode, StringTypeNode,
    };

    #[test]
    fn implicit() {
        assert_type!(
            { array(number(u8), fixed_count(3)) },
            ArrayTypeNode::new(NumberTypeNode::le(U8), FixedCountNode::new(3)).into()
        );
    }

    #[test]
    fn bare_integer_count() {
        assert_type!(
            { array(number(u8), 3) },
            ArrayTypeNode::new(NumberTypeNode::le(U8), FixedCountNode::new(3)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { array(item = number(u8), count = fixed_count(3)) },
            ArrayTypeNode::new(NumberTypeNode::le(U8), FixedCountNode::new(3)).into()
        );
    }

    #[test]
    fn prefixed_count() {
        assert_type!(
            { array(string, prefixed_count(number(u32))) },
            ArrayTypeNode::new(
                StringTypeNode::utf8(),
                PrefixedCountNode::new(NumberTypeNode::le(U32))
            )
            .into()
        );
    }

    #[test]
    fn remainder_count() {
        assert_type!(
            { array(number(u8), remainder_count) },
            ArrayTypeNode::new(NumberTypeNode::le(U8), RemainderCountNode::new()).into()
        );
    }

    #[test]
    fn item_missing() {
        assert_type_err!({ array(fixed_count(3)) }, "item is missing");
    }

    #[test]
    fn count_missing() {
        assert_type_err!({ array(number(u8)) }, "count is missing");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ array(foo = 42) }, "unrecognized attribute");
    }
}
