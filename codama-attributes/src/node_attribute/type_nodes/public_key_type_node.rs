use crate::NodeAttributeParse;
use codama_nodes::{Node, PublicKeyTypeNode};
use codama_syn_helpers::syn_traits::*;

impl NodeAttributeParse for PublicKeyTypeNode {
    fn from_meta(meta: &syn::meta::ParseNestedMeta) -> syn::Result<Node> {
        let arg = meta.input.fork_arg()?;
        if !arg.is_end_of_arg() && !arg.is_empty_group() {
            return Err(meta.error("public_key_type does not accept any input"));
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
        assert_node!(#[node(public_key_type)], PublicKeyTypeNode::new().into());
        assert_node!(#[node(public_key_type())], PublicKeyTypeNode::new().into());
    }

    #[test]
    fn unexpected_input() {
        assert_node_err!(#[node(public_key_type(unexpected))], "public_key_type does not accept any input");
        assert_node_err!(#[node(public_key_type(foo = 42))], "public_key_type does not accept any input");
    }
}
