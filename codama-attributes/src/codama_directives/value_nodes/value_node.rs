use crate::utils::FromMeta;
use codama_nodes::{BooleanValueNode, NumberValueNode, StringValueNode, ValueNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for ValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "todo" => Ok(BooleanValueNode::from_meta(meta)?.into()),
            _ => BooleanValueNode::from_meta(meta)
                .map(ValueNode::from)
                .or(StringValueNode::from_meta(meta).map(ValueNode::from))
                .or(NumberValueNode::from_meta(meta).map(ValueNode::from))
                .map_err(|_| meta.error("unrecognized value")),
        }
    }
}
