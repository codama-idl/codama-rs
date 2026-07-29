use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{RemainderOptionTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for RemainderOptionTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("remainder_option")?.as_path_list()?;
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

        Ok(RemainderOptionTypeNode::new(item.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{NumberTypeNode, StringTypeNode, U8};

    #[test]
    fn implicit() {
        assert_type!(
            { remainder_option(number(u8)) },
            RemainderOptionTypeNode::new(NumberTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { remainder_option(item = number(u8)) },
            RemainderOptionTypeNode::new(NumberTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn nested() {
        assert_type!(
            { remainder_option(size_prefix(string, number(u8))) },
            RemainderOptionTypeNode::new(codama_nodes::SizePrefixTypeNode::new(
                StringTypeNode::utf8(),
                NumberTypeNode::le(U8)
            ))
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
            { remainder_option(number(u8), string) },
            "item is already set"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ remainder_option(foo = 42) }, "unrecognized attribute");
    }
}
