use crate::FromMeta;
use codama_nodes::{Node, PublicKeyTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for PublicKeyTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Node> {
        if !meta.is_path_or_empty_list() {
            return Err(meta.error("public_key_type does not accept any input"));
        }
        Ok(PublicKeyTypeNode::new().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_node, assert_node_err};

    #[test]
    fn ok() {
        assert_node!({ public_key_type }, PublicKeyTypeNode::new().into());
        assert_node!({ public_key_type() }, PublicKeyTypeNode::new().into());
    }

    #[test]
    fn unexpected_input() {
        assert_node_err!(
            { public_key_type(unexpected) },
            "public_key_type does not accept any input"
        );
        assert_node_err!(
            { public_key_type(foo = 42) },
            "public_key_type does not accept any input"
        );
    }
}
