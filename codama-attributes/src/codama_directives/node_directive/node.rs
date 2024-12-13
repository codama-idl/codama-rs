use super::FromMeta;
use codama_nodes::*;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for Node {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let path = meta.path()?;
        Ok(match path.to_string().as_str() {
            // Type nodes.
            "boolean_type" => BooleanTypeNode::from_meta(meta)?.into(),
            "fixed_size_type" => FixedSizeTypeNode::<TypeNode>::from_meta(meta)?.into(),
            "number_type" => NumberTypeNode::from_meta(meta)?.into(),
            "public_key_type" => PublicKeyTypeNode::from_meta(meta)?.into(),
            _ => return Err(path.error("unrecognized node")),
        })
    }
}
