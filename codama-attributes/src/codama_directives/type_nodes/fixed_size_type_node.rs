use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{FixedSizeTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for FixedSizeTypeNode<TypeNode> {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("fixed_size")?.as_path_list()?;
        let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
        let mut size: SetOnce<usize> = SetOnce::new("size");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "type" => {
                let node = TypeNode::from_meta(meta.as_value()?)?;
                r#type.set(node, meta)
            }
            "size" => size.set(meta.as_value()?.as_expr()?.as_unsigned_integer()?, meta),
            _ => {
                if meta.is_path_or_list() {
                    return r#type.set(TypeNode::from_meta(meta)?, meta);
                }
                if let Ok(expr) = meta.as_expr() {
                    return size.set(expr.as_unsigned_integer()?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        let r#type = r#type.take(meta)?;
        Ok(Self::new(r#type, size.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::BooleanTypeNode;

    #[test]
    fn explicit() {
        assert_type!(
            { fixed_size(type = boolean, size = 42)},
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
        assert_type!(
            { fixed_size(size = 42, type = boolean)},
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
    }

    #[test]
    fn implicit() {
        assert_type!(
            { fixed_size(boolean, 42) },
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
        assert_type!(
            { fixed_size(42, boolean) },
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
    }

    #[test]
    fn unrecognized_type() {
        assert_type_err!({ fixed_size(unrecognized) }, "unrecognized type");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ fixed_size(foo = 42) }, "unrecognized attribute");
    }
}
