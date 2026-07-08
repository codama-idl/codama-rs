use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{TypeNode, ZeroableOptionTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for ZeroableOptionTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("zeroable_option")?.as_path_list()?;
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

        Ok(ZeroableOptionTypeNode::new(item.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{NumberTypeNode, U8};

    #[test]
    fn implicit() {
        assert_type!(
            { zeroable_option(number(u8)) },
            ZeroableOptionTypeNode::new(NumberTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { zeroable_option(item = number(u8)) },
            ZeroableOptionTypeNode::new(NumberTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ zeroable_option(foo = 42) }, "unrecognized attribute");
    }
}
