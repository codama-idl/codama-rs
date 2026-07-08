use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{NumberTypeNode, OptionTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for OptionTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("option")?.as_path_list()?;
        let mut item: SetOnce<TypeNode> = SetOnce::new("item");
        let mut prefix: SetOnce<NumberTypeNode> = SetOnce::new("prefix");
        let mut fixed: SetOnce<bool> = SetOnce::new("fixed");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "item" => item.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            "prefix" => prefix.set(NumberTypeNode::from_meta(meta.as_value()?)?, meta),
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
            node.prefix = prefix.into();
        }
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{NumberTypeNode, U16, U8};

    #[test]
    fn implicit() {
        assert_type!(
            { option(number(u8)) },
            OptionTypeNode::new(NumberTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { option(item = number(u8)) },
            OptionTypeNode::new(NumberTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn fixed_flag() {
        assert_type!(
            { option(number(u8), fixed) },
            OptionTypeNode::fixed(NumberTypeNode::le(U8)).into()
        );
        assert_type!(
            { option(item = number(u8), fixed = true) },
            OptionTypeNode::fixed(NumberTypeNode::le(U8)).into()
        );
    }

    #[test]
    fn custom_prefix() {
        let mut expected = OptionTypeNode::new(NumberTypeNode::le(U8));
        expected.prefix = NumberTypeNode::le(U16).into();
        assert_type!(
            { option(number(u8), prefix = number(u16, le)) },
            expected.into()
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
