use crate::utils::FromMeta;
use codama_nodes::{BooleanValueNode, ValueNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for ValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "todo" => Ok(BooleanValueNode::from_meta(meta)?.into()),
            _ => {
                if let Ok(value) = BooleanValueNode::from_meta(meta) {
                    Ok(value.into())
                } else {
                    Err(meta.error("unrecognized value"))
                }
            }
        }
    }
}
