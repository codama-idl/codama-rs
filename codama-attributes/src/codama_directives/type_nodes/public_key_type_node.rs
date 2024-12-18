use crate::utils::FromMeta;
use codama_nodes::PublicKeyTypeNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for PublicKeyTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        if !meta.is_path_or_empty_list() {
            return Err(meta.error("public_key does not accept any input"));
        }
        Ok(Self::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_node, assert_node_err};

    #[test]
    fn ok() {
        assert_node!({ public_key }, PublicKeyTypeNode::new().into());
        assert_node!({ public_key() }, PublicKeyTypeNode::new().into());
    }

    #[test]
    fn unexpected_input() {
        assert_node_err!(
            { public_key(unexpected) },
            "public_key does not accept any input"
        );
        assert_node_err!(
            { public_key(foo = 42) },
            "public_key does not accept any input"
        );
    }
}