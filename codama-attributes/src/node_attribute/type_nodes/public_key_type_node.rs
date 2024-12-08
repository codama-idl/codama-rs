use crate::NodeAttributeParse;
use codama_errors::CodamaResult;
use codama_nodes::{Node, PublicKeyTypeNode};

impl NodeAttributeParse for PublicKeyTypeNode {
    fn from_meta(meta: &syn::meta::ParseNestedMeta) -> CodamaResult<Node> {
        if !meta.input.is_empty() {
            return Err(meta.error("publicKeyTypeNode does accept any input").into());
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
    #[ignore]
    fn ok() {
        assert_node!(#[node(publicKeyTypeNode)], PublicKeyTypeNode::new().into());
        assert_node!(#[node(publicKeyTypeNode())], PublicKeyTypeNode::new().into());
    }

    #[test]
    #[ignore]
    fn unexpected_input() {
        assert_node_err!(#[node(publicKeyTypeNode(unexpected))], "publicKeyTypeNode does accept any input");
    }
}
