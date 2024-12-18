use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{BooleanTypeNode, NestedTypeNode, NumberTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for BooleanTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let mut size: SetOnce<TypeNode> = SetOnce::<TypeNode>::new("size");
        if meta.is_path_or_empty_list() {
            return Ok(BooleanTypeNode::default().into());
        }
        meta.as_path_list()?
            .each(|ref meta| match (meta.path_str().as_str(), meta) {
                ("size", _) => {
                    let node = TypeNode::from_meta(&meta.as_path_value()?.value)?;
                    size.set(node, meta)
                }
                (_, m) if m.is_path_or_list() => size.set(TypeNode::from_meta(meta)?, meta),
                _ => Err(meta.error("unrecognized attribute")),
            })?;
        let size = match NestedTypeNode::<NumberTypeNode>::try_from(size.take(meta)?) {
            Ok(node) => node,
            Err(_) => return Err(meta.error("size must be a NumberTypeNode")),
        };
        Ok(BooleanTypeNode::new(size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::NumberFormat::U32;

    #[test]
    fn default() {
        assert_type!({ boolean }, BooleanTypeNode::default().into());
        assert_type!({ boolean() }, BooleanTypeNode::default().into());
    }

    #[test]
    fn implicit() {
        assert_type!(
            { boolean(number(u32)) },
            BooleanTypeNode::new(NumberTypeNode::le(U32)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { boolean(size = number(u32)) },
            BooleanTypeNode::new(NumberTypeNode::le(U32)).into()
        );
    }

    #[test]
    fn unrecognized_type() {
        assert_type_err!({ boolean(unrecognized) }, "unrecognized type");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ boolean(foo = 42) }, "unrecognized attribute");
    }
}
