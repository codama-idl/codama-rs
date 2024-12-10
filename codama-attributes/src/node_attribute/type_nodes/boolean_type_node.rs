use crate::{utils::SetOnce, NodeAttributeParse};
use codama_nodes::{BooleanTypeNode, NestedTypeNode, Node, NumberTypeNode};
use codama_syn_helpers::{syn_traits::*, AttributeMeta};

impl NodeAttributeParse for BooleanTypeNode {
    fn from_meta(meta: &AttributeMeta) -> syn::Result<Node> {
        let mut size = SetOnce::<NestedTypeNode<NumberTypeNode>>::new("size");
        if meta.input.is_end_of_arg() || meta.input.is_empty_group() {
            meta.input.consume_arg()?;
            return Ok(BooleanTypeNode::default().into());
        }
        meta.parse_metas(|ref meta| {
            match meta.input.fork().parse::<syn::Path>()?.last_str().as_str() {
                "size" => {
                    meta.input.parse::<syn::Path>()?;
                    meta.input.parse::<syn::Token![=]>()?;
                    let node = Node::from_meta(meta)?;
                    let node = match NestedTypeNode::<NumberTypeNode>::try_from(node) {
                        Ok(node) => node,
                        Err(_) => return Err(meta.error("size must be a NumberTypeNode")),
                    };
                    size.set(node, meta)?;
                    Ok(())
                }
                _ => {
                    let node = Node::from_meta(meta)?;
                    let node = match NestedTypeNode::<NumberTypeNode>::try_from(node) {
                        Ok(node) => node,
                        Err(_) => return Err(meta.error("size must be a NumberTypeNode")),
                    };
                    size.set(node, meta)?;
                    Ok(())
                }
            }
        })?;
        Ok(BooleanTypeNode::new(size.take(meta)?).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_node, assert_node_err, NodeAttribute};
    use codama_nodes::NumberFormat::U32;
    use codama_syn_helpers::syn_build;
    use quote::quote;

    #[test]
    fn default_size() {
        assert_node!(#[node(boolean_type)], BooleanTypeNode::default().into());
        assert_node!(#[node(boolean_type())], BooleanTypeNode::default().into());
    }

    #[test]
    fn custom_size() {
        assert_node!(#[node(boolean_type(number_type(u32, be)))], BooleanTypeNode::new(NumberTypeNode::be(U32)).into());
        assert_node!(#[node(boolean_type(size = number_type(u32, be)))], BooleanTypeNode::new(NumberTypeNode::be(U32)).into());
    }

    #[test]
    fn unrecognized_node() {
        assert_node_err!(#[node(boolean_type(unrecognized))], "unrecognized node");
        assert_node_err!(#[node(boolean_type(foo = 42))], "unrecognized node");
    }
}
