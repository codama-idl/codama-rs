use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{RemainderOptionTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for RemainderOptionTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        Ok(RemainderOptionTypeNode::new(parse_option_item(
            meta,
            "remainder_option",
        )?))
    }
}

/// Parses a `remainder_option(item)` or `zeroable_option(item)` directive, named `directive`, into its
/// item type.
pub(crate) fn parse_option_item(meta: &Meta, directive: &str) -> syn::Result<TypeNode> {
    let pl = meta.assert_directive(directive)?.as_path_list()?;
    let mut item: SetOnce<TypeNode> = SetOnce::new("item");

    pl.each(|ref meta| match meta.path_str().as_str() {
        "item" => item.set(TypeNode::from_meta(meta.as_value()?)?, meta),
        _ => {
            if meta.is_path_or_list() {
                return item.set(TypeNode::from_meta(meta)?, meta);
            }
            Err(meta.error("unrecognized attribute"))
        }
    })?;

    item.take(meta)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{assert_type, assert_type_err};
    use codama_nodes::{
        HasTransforms, IntegerTypeNode, SizePrefixTransformNode, StringTypeNode, U8,
    };

    #[test]
    fn implicit() {
        assert_type!(
            { remainder_option(integer(u8)) },
            RemainderOptionTypeNode::new(IntegerTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { remainder_option(item = integer(u8)) },
            RemainderOptionTypeNode::new(IntegerTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn nested() {
        assert_type!(
            { remainder_option(size_prefix(string, integer(u8))) },
            RemainderOptionTypeNode::new(
                StringTypeNode::utf8()
                    .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U8)))
            )
            .into()
        );
    }

    #[test]
    fn item_missing() {
        assert_type_err!({ remainder_option() }, "item is missing");
    }

    #[test]
    fn already_set() {
        assert_type_err!(
            { remainder_option(integer(u8), string) },
            "item is already set"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ remainder_option(foo = 42) }, "unrecognized attribute");
    }
}
