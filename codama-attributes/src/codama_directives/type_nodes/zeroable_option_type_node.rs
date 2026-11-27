use super::remainder_option_type_node::parse_option_item;
use crate::utils::FromMeta;
use codama_nodes::ZeroableOptionTypeNode;
use codama_syn_helpers::Meta;

impl FromMeta for ZeroableOptionTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        Ok(ZeroableOptionTypeNode::new(parse_option_item(
            meta,
            "zeroable_option",
        )?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{assert_type, assert_type_err};
    use codama_nodes::{IntegerTypeNode, U8};

    #[test]
    fn implicit() {
        assert_type!(
            { zeroable_option(integer(u8)) },
            ZeroableOptionTypeNode::new(IntegerTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { zeroable_option(item = integer(u8)) },
            ZeroableOptionTypeNode::new(IntegerTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ zeroable_option(foo = 42) }, "unrecognized attribute");
    }
}
