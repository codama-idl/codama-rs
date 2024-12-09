use crate::NodeAttributeParse;
use codama_nodes::{Node, PublicKeyTypeNode};
use codama_syn_helpers::syn_traits::*;

impl NodeAttributeParse for PublicKeyTypeNode {
    fn from_meta(meta: &syn::meta::ParseNestedMeta) -> syn::Result<Node> {
        if !meta.input.is_end_of_arg() && !meta.input.consume_empty_group() {
            return Err(meta.error("publicKeyTypeNode does not accept any input"));
        }
        Ok(PublicKeyTypeNode::new().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_node, assert_node_err, NodeAttribute};
    use codama_syn_helpers::syn_build;
    use quote::quote;

    #[test]
    fn ok() {
        assert_node!(#[node(publicKeyTypeNode)], PublicKeyTypeNode::new().into());
        assert_node!(#[node(publicKeyTypeNode())], PublicKeyTypeNode::new().into());
    }

    #[test]
    fn unexpected_input() {
        assert_node_err!(#[node(publicKeyTypeNode(unexpected))], "publicKeyTypeNode does not accept any input");
        assert_node_err!(#[node(publicKeyTypeNode(foo = 42))], "publicKeyTypeNode does not accept any input");
    }
}
