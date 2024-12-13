use crate::{utils::SetOnce, FromMeta};
use codama_nodes::{BooleanTypeNode, NestedTypeNode, Node, NumberTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for BooleanTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let mut size: SetOnce<Node> = SetOnce::<Node>::new("size");
        if meta.is_path_or_empty_list() {
            return Ok(BooleanTypeNode::default().into());
        }
        meta.as_list()?
            .each(|ref meta| match (meta.path_str().as_str(), meta) {
                ("size", _) => {
                    let node = Node::from_meta(&meta.value_as_meta()?)?;
                    size.set(node, meta)
                }
                (_, Meta::List(_) | Meta::Path(_)) => size.set(Node::from_meta(meta)?, meta),
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
    use crate::{assert_node, assert_node_err};
    use codama_nodes::NumberFormat::U32;

    #[test]
    fn default() {
        assert_node!({ boolean_type }, BooleanTypeNode::default().into());
        assert_node!({ boolean_type() }, BooleanTypeNode::default().into());
    }

    #[test]
    fn implicit() {
        assert_node!(
            { boolean_type(number_type(u32, be)) },
            BooleanTypeNode::new(NumberTypeNode::be(U32)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_node!(
            { boolean_type(size = number_type(u32, be)) },
            BooleanTypeNode::new(NumberTypeNode::be(U32)).into()
        );
    }

    #[test]
    fn unrecognized_node() {
        assert_node_err!({ boolean_type(unrecognized) }, "unrecognized node");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_node_err!({ boolean_type(foo = 42) }, "unrecognized attribute");
    }
}
