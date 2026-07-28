use super::nested_number::nested_number;
use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{NestedTypeNode, NumberTypeNode, SolAmountTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for SolAmountTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("sol_amount")?.as_path_list()?;
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

        Ok(SolAmountTypeNode::new(number.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::NumberFormat::U64;

    #[test]
    fn implicit() {
        assert_type!(
            { sol_amount(number(u64)) },
            SolAmountTypeNode::new(NumberTypeNode::le(U64)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { sol_amount(number = number(u64)) },
            SolAmountTypeNode::new(NumberTypeNode::le(U64)).into()
        );
    }

    #[test]
    fn invalid_number() {
        assert_type_err!(
            { sol_amount(public_key) },
            "number must be a NumberTypeNode"
        );
    }

    #[test]
    fn number_missing() {
        assert_type_err!({ sol_amount() }, "number is missing");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ sol_amount(foo = 42) }, "unrecognized attribute");
    }
}
