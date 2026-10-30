use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{IntegerTypeNode, OptionTypeNode, TypeNode, TypeNodeTrait};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for OptionTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("option")?.as_path_list()?;
        let mut item: SetOnce<TypeNode> = SetOnce::new("item");
        let mut prefix: SetOnce<IntegerTypeNode> = SetOnce::new("prefix");
        let mut fixed: SetOnce<bool> = SetOnce::new("fixed");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "item" => item.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            "prefix" => {
                let value = meta.as_value()?;
                match IntegerTypeNode::try_from_type_node(TypeNode::from_meta(value)?) {
                    Ok(node) => prefix.set(node, meta),
                    Err(_) => Err(value.error("prefix must be an IntegerTypeNode")),
                }
            }
            // `fixed` (bare flag) or `fixed = true/false`.
            "fixed" => {
                let value = match meta.as_value() {
                    Ok(value) => value.as_expr()?.as_bool()?,
                    Err(_) => true,
                };
                fixed.set(value, meta)
            }
            _ => {
                if meta.is_path_or_list() {
                    return item.set(TypeNode::from_meta(meta)?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        let item = item.take(meta)?;
        let mut node = if fixed.option().unwrap_or(false) {
            OptionTypeNode::fixed(item)
        } else {
            OptionTypeNode::new(item)
        };
        if let Some(prefix) = prefix.option() {
            node.prefix = prefix;
        }
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{U16, U8};

    #[test]
    fn implicit() {
        assert_type!(
            { option(integer(u8)) },
            OptionTypeNode::new(IntegerTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { option(item = integer(u8)) },
            OptionTypeNode::new(IntegerTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn fixed_flag() {
        assert_type!(
            { option(integer(u8), fixed) },
            OptionTypeNode::fixed(IntegerTypeNode::le(U8)).into()
        );
        assert_type!(
            { option(item = integer(u8), fixed = true) },
            OptionTypeNode::fixed(IntegerTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn custom_prefix() {
        let mut expected = OptionTypeNode::new(IntegerTypeNode::le(U8));
        expected.prefix = IntegerTypeNode::le(U16);
        assert_type!(
            { option(integer(u8), prefix = integer(u16, le)) },
            expected.into()
        );
    }

    #[test]
    fn invalid_prefix() {
        assert_type_err!(
            { option(integer(u8), prefix = string) },
            "prefix must be an IntegerTypeNode"
        );
        assert_type_err!(
            { option(integer(u8), prefix = float(f32)) },
            "prefix must be an IntegerTypeNode"
        );
    }

    #[test]
    fn unrecognized_type() {
        assert_type_err!({ option(unrecognized) }, "unrecognized type");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ option(foo = 42) }, "unrecognized attribute");
    }
}
