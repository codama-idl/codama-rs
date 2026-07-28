use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{ConstantValueNode, HiddenSuffixTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for HiddenSuffixTypeNode<TypeNode> {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("hidden_suffix")?.as_path_list()?;
        let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
        let mut suffix: Vec<ConstantValueNode> = Vec::new();

        pl.each(|ref meta| match meta.path_str().as_str() {
            "type" => r#type.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            // Every `constant(...)` is appended, so the suffix keeps its order.
            "constant" => {
                suffix.push(ConstantValueNode::from_meta(meta)?);
                Ok(())
            }
            _ => {
                if meta.is_path_or_list() {
                    return r#type.set(TypeNode::from_meta(meta)?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        Ok(HiddenSuffixTypeNode::new(r#type.take(meta)?, suffix))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{NumberFormat::U8, NumberTypeNode, NumberValueNode, StringTypeNode};

    fn constant(value: u64) -> ConstantValueNode {
        ConstantValueNode::new(NumberTypeNode::le(U8), NumberValueNode::new(value))
    }

    #[test]
    fn single_constant() {
        assert_type!(
            { hidden_suffix(string, constant(number(u8), 1)) },
            HiddenSuffixTypeNode::new(StringTypeNode::utf8(), vec![constant(1)]).into()
        );
    }

    #[test]
    fn multiple_constants_keep_their_order() {
        assert_type!(
            { hidden_suffix(string, constant(number(u8), 1), constant(number(u8), 2)) },
            HiddenSuffixTypeNode::new(StringTypeNode::utf8(), vec![constant(1), constant(2)])
                .into()
        );
    }

    #[test]
    fn type_missing() {
        assert_type_err!(
            { hidden_suffix(constant(number(u8), 1)) },
            "type is missing"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ hidden_suffix(foo = 42) }, "unrecognized attribute");
    }
}
