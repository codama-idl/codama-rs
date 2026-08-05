use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{DateTimeTypeNode, NestedTypeNode, NumberTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for DateTimeTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("date_time")?.as_path_list()?;
        let mut number: SetOnce<NestedTypeNode<NumberTypeNode>> = SetOnce::new("number");

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
