use super::nested_number::nested_number;
use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{DateTimeTypeNode, NestedTypeNode, NumberTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for DateTimeTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("date_time")?.as_path_list()?;
        let mut number: SetOnce<NestedTypeNode<NumberTypeNode>> = SetOnce::new("number");

        pl.each(|ref meta| {
            // A positional `number(u64)` shares its path with the `number` field,
            // so lists and bare paths are always read as the type node.
            if meta.is_path_or_list() {
                return number.set(nested_number(meta, "number")?, meta);
            }
            match meta.path_str().as_str() {
                "number" => number.set(nested_number(meta.as_value()?, "number")?, meta),
                _ => Err(meta.error("unrecognized attribute")),
            }
        })?;

        Ok(DateTimeTypeNode::new(number.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{FixedSizeTypeNode, NumberFormat::U64};

    #[test]
    fn implicit() {
        assert_type!(
            { date_time(number(u64)) },
            DateTimeTypeNode::new(NumberTypeNode::le(U64)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { date_time(number = number(u64)) },
            DateTimeTypeNode::new(NumberTypeNode::le(U64)).into()
        );
    }

    #[test]
    fn nested_number() {
        assert_type!(
            { date_time(fixed_size(number(u64), 8)) },
            DateTimeTypeNode::new(FixedSizeTypeNode::<NestedTypeNode<NumberTypeNode>>::new(
                NumberTypeNode::le(U64),
                8
            ))
            .into()
        );
    }

    #[test]
    fn invalid_number() {
        assert_type_err!({ date_time(string) }, "number must be a NumberTypeNode");
    }

    #[test]
    fn number_missing() {
        assert_type_err!({ date_time() }, "number is missing");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ date_time(foo = 42) }, "unrecognized attribute");
    }
}
