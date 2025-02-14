use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{FixedSizeTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for FixedSizeTypeNode<TypeNode> {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
        let mut size: SetOnce<usize> = SetOnce::new("size");
        meta.as_path_list()?
            .each(|ref meta| match (meta.path_str().as_str(), meta) {
                ("type", _) => {
                    let node = TypeNode::from_meta(&meta.as_path_value()?.value)?;
                    r#type.set(node, meta)
                }
                ("size", _) => size.set(
                    meta.as_path_value()?
                        .value
                        .as_expr()?
                        .as_literal_integer()?,
                    meta,
                ),
                (_, m) if m.is_path_or_list() => r#type.set(TypeNode::from_meta(meta)?, meta),
                (_, Meta::Expr(expr)) => size.set(expr.as_literal_integer()?, meta),
                _ => Err(meta.error("unrecognized attribute")),
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
