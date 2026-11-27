use super::array_type_node::parse_collection_type;
use crate::utils::FromMeta;
use codama_nodes::SetTypeNode;
use codama_syn_helpers::Meta;

impl FromMeta for SetTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let (item, count) = parse_collection_type(meta, "set")?;
        Ok(SetTypeNode::new(item, count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{assert_type, assert_type_err};
    use codama_nodes::{
        FixedCountNode,
        IntegerFormat::{U32, U8},
        IntegerTypeNode, PrefixedCountNode,
    };

    #[test]
    fn implicit() {
        assert_type!(
            { set(integer(u8), fixed_count(3)) },
            SetTypeNode::new(IntegerTypeNode::le(U8), FixedCountNode::new(3)).into()
        );
    }

    #[test]
    fn bare_integer_count() {
        assert_type!(
            { set(integer(u8), 3) },
            SetTypeNode::new(IntegerTypeNode::le(U8), FixedCountNode::new(3)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { set(item = integer(u8), count = prefixed_count(integer(u32))) },
            SetTypeNode::new(
                IntegerTypeNode::le(U8),
                PrefixedCountNode::new(IntegerTypeNode::le(U32))
            )
            .into()
        );
    }

    #[test]
    fn count_missing() {
        assert_type_err!({ set(integer(u8)) }, "count is missing");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ set(foo = 42) }, "unrecognized attribute");
    }
}
