use crate::{utils::SetOnce, NodeAttributeParse};
use codama_nodes::{BooleanTypeNode, NestedTypeNode, Node, NumberTypeNode};
use codama_syn_helpers::{
    syn_traits::{MetaList, Path},
    Meta,
};

impl NodeAttributeParse for BooleanTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Node> {
        let mut size = SetOnce::<NestedTypeNode<NumberTypeNode>>::new("size");
        if meta.is_path_or_empty_list() {
            return Ok(BooleanTypeNode::default().into());
        }
        meta.as_list()?
            .parse_metas(|ref meta| match meta.path()?.last_str().as_str() {
                "size" => {
                    let name_list = meta.as_name_list()?;
                    let node = Node::from_meta(&Meta::List(name_list.list.clone()))?;
                    let node = match NestedTypeNode::<NumberTypeNode>::try_from(node) {
                        Ok(node) => node,
                        Err(_) => {
                            return Err(syn::Error::new_spanned(
                                &name_list.list,
                                "size must be a NumberTypeNode",
                            ))
                        }
                    };
                    size.set(node, meta)?;
                    Ok(())
                }
                _ => {
                    let node = Node::from_meta(meta)?;
                    let node = match NestedTypeNode::<NumberTypeNode>::try_from(node) {
                        Ok(node) => node,
                        Err(_) => {
                            return Err(syn::Error::new_spanned(
                                meta,
                                "size must be a NumberTypeNode",
                            ))
                        }
                    };
                    size.set(node, meta)?;
                    Ok(())
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
