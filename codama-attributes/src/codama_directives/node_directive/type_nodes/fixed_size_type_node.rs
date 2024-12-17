use crate::{utils::SetOnce, FromMeta};
use codama_nodes::{FixedSizeTypeNode, Node, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for FixedSizeTypeNode<TypeNode> {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let mut r#type: SetOnce<Node> = SetOnce::<Node>::new("type");
        let mut size: SetOnce<usize> = SetOnce::<usize>::new("size");
        meta.as_list()?
            .each(|ref meta| match (meta.path_str().as_str(), meta) {
                ("type", _) => {
                    let node = Node::from_meta(&meta.as_label()?.value)?;
                    r#type.set(node, meta)
                }
                ("size", _) => size.set(
                    meta.as_label()?.value.as_expr()?.as_literal_integer()?,
                    meta,
                ),
                (_, Meta::List(_) | Meta::Path(_)) => r#type.set(Node::from_meta(meta)?, meta),
                (_, Meta::Expr(expr)) => size.set(expr.as_literal_integer()?, meta),
                _ => Err(meta.error("unrecognized attribute")),
            })?;
        let r#type = match TypeNode::try_from(r#type.take(meta)?) {
            Ok(node) => node,
            Err(_) => return Err(meta.error("type must be a TypeNode")),
        };
        Ok(Self::new(r#type, size.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_node, assert_node_err};
    use codama_nodes::BooleanTypeNode;

    #[test]
    fn explicit() {
        assert_node!(
            { fixed_size_type(type = boolean_type, size = 42)},
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
        assert_node!(
            { fixed_size_type(size = 42, type = boolean_type)},
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
    }

    #[test]
    fn implicit() {
        assert_node!(
            { fixed_size_type(boolean_type, 42) },
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
        assert_node!(
            { fixed_size_type(42, boolean_type) },
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
    }

    #[test]
    fn unrecognized_node() {
        assert_node_err!({ fixed_size_type(unrecognized) }, "unrecognized node");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_node_err!({ fixed_size_type(foo = 42) }, "unrecognized attribute");
    }
}
