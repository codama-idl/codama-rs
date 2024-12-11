use crate::{utils::SetOnce, NodeAttributeParse};
use codama_nodes::{FixedSizeTypeNode, Node, TypeNode, TypeNodeUnionTrait};
use codama_syn_helpers::{syn_traits::*, Meta};

impl<T: TypeNodeUnionTrait> NodeAttributeParse for FixedSizeTypeNode<T> {
    fn from_meta(meta: &Meta) -> syn::Result<Node> {
        let mut r#type: SetOnce<Node> = SetOnce::<Node>::new("type");
        let mut size: SetOnce<usize> = SetOnce::<usize>::new("size");
        meta.as_list()?
            .parse_metas(|ref meta| match meta.path()?.to_string().as_str() {
                "item" => {
                    let node = Node::from_meta(&meta.value_as_meta()?)?;
                    r#type.set(node, meta)
                }
                "size" => size.set(meta.as_name_value()?.value.as_literal_integer()?, meta),
                _ => r#type.set(Node::from_meta(meta)?, meta),
            })?;
        let r#type = match TypeNode::try_from(r#type.take(meta)?) {
            Ok(node) => node,
            Err(_) => return Err(meta.error("type must be a TypeNode")),
        };
        Ok(FixedSizeTypeNode::new(r#type, size.take(meta)?).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_node, assert_node_err, NodeAttribute};
    use codama_nodes::BooleanTypeNode;
    use codama_syn_helpers::syn_build;
    use quote::quote;

    #[test]
    fn explicit_type() {
        assert_node!(
            #[node(fixed_size_type(item = boolean_type, size = 42))],
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
        assert_node!(
            #[node(fixed_size_type(size = 42, item = boolean_type))],
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
    }

    #[test]
    fn implicit_type() {
        assert_node!(
            #[node(fixed_size_type(boolean_type, size = 42))],
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
        assert_node!(
            #[node(fixed_size_type(size = 42, boolean_type))],
            FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into()
        );
    }

    #[test]
    fn unrecognized_node() {
        assert_node_err!(#[node(fixed_size_type(unrecognized))], "unrecognized node");
        assert_node_err!(#[node(fixed_size_type(foo = 42))], "unrecognized node");
    }
}
