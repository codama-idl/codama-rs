use crate::codama_directives::value_nodes::sysvar::public_key_value_node_from_sysvar;
use crate::utils::FromMeta;
use codama_nodes::{
    BooleanValueNode, NumberValueNode, PublicKeyValueNode, StringValueNode, ValueNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for ValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "public_key" => PublicKeyValueNode::from_meta(meta).map(Self::from),
            "sysvar" => public_key_value_node_from_sysvar(meta).map(Self::from),
            _ => BooleanValueNode::from_meta(meta)
                .map(Self::from)
                .or(StringValueNode::from_meta(meta).map(Self::from))
                .or(NumberValueNode::from_meta(meta).map(Self::from))
                .map_err(|_| meta.error("unrecognized value")),
        }
    }
}
