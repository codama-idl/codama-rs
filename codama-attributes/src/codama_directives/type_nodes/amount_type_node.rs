use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{AmountTypeNode, NestedTypeNode, NumberTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for AmountTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("amount")?.as_path_list()?;
        let mut number: SetOnce<NestedTypeNode<NumberTypeNode>> = SetOnce::new("number");
        let mut decimals: SetOnce<u32> = SetOnce::new("decimals");
        let mut unit: SetOnce<String> = SetOnce::new("unit");

        pl.each(|ref meta| {
            // A positional `number(u64)` shares its path with the `number` field,
            // so lists and bare paths are always read as the type node.
            if meta.is_path_or_list() {
                let node = TypeNode::from_meta(meta)?;
                let number_node = match NestedTypeNode::<NumberTypeNode>::try_from(node) {
                    Ok(node) => node,
                    Err(_) => return Err(meta.error("number must be a NumberTypeNode")),
                };
                return number.set(number_node, meta);
            }
            match meta.path_str().as_str() {
                "number" => {
                    let value = meta.as_value()?;
                    let node = TypeNode::from_meta(value)?;
                    let number_node = match NestedTypeNode::<NumberTypeNode>::try_from(node) {
                        Ok(node) => node,
                        Err(_) => return Err(value.error("number must be a NumberTypeNode")),
                    };
                    number.set(number_node, meta)
                }
                "decimals" => {
                    decimals.set(meta.as_value()?.as_expr()?.as_unsigned_integer()?, meta)
                }
                "unit" => unit.set(meta.as_value()?.as_expr()?.as_string()?, meta),
                _ => {
                    // The remaining positional args fill `decimals` then `unit`.
                    if let Ok(expr) = meta.as_expr() {
                        return match decimals.is_set() {
                            false => decimals.set(expr.as_unsigned_integer()?, meta),
                            true => unit.set(expr.as_string()?, meta),
                        };
                    }
                    Err(meta.error("unrecognized attribute"))
                }
            }
        })?;

        Ok(AmountTypeNode::new(
            number.take(meta)?,
            decimals.option().unwrap_or(0),
            unit.option(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::NumberFormat::U64;

    #[test]
    fn number_only() {
        assert_type!(
            { amount(number(u64)) },
            AmountTypeNode::new(NumberTypeNode::le(U64), 0, None).into()
        );
    }

    #[test]
    fn implicit() {
        assert_type!(
            { amount(number(u64), 9, "SOL") },
            AmountTypeNode::new(NumberTypeNode::le(U64), 9, Some("SOL".to_string())).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { amount(number = number(u64), decimals = 9, unit = "SOL") },
            AmountTypeNode::new(NumberTypeNode::le(U64), 9, Some("SOL".to_string())).into()
        );
    }

    #[test]
    fn positional_args_are_ordered() {
        assert_type_err!(
            { amount(number(u64), "SOL", 9) },
            "expected an unsigned integer"
        );
    }

    #[test]
    fn decimals_without_unit() {
        assert_type!(
            { amount(number(u64), 6) },
            AmountTypeNode::new(NumberTypeNode::le(U64), 6, None).into()
        );
    }

    #[test]
    fn invalid_number() {
        assert_type_err!({ amount(string) }, "number must be a NumberTypeNode");
    }

    #[test]
    fn number_missing() {
        assert_type_err!({ amount(decimals = 9) }, "number is missing");
    }

    #[test]
    fn already_set() {
        assert_type_err!(
            { amount(number(u64), decimals = 9, decimals = 6) },
            "decimals is already set"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ amount(foo = 42) }, "unrecognized attribute");
    }
}
