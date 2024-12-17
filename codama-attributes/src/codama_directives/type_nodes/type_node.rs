use crate::utils::FromMeta;
use codama_nodes::{
    BooleanTypeNode, FixedSizeTypeNode, NumberTypeNode, PublicKeyTypeNode, TypeNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for TypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "boolean" => Ok(BooleanTypeNode::from_meta(meta)?.into()),
            "fixed_size" => Ok(FixedSizeTypeNode::from_meta(meta)?.into()),
            "number" => Ok(NumberTypeNode::from_meta(meta)?.into()),
            "public_key" => Ok(PublicKeyTypeNode::from_meta(meta)?.into()),
            _ => Err(meta.error("unrecognized type")),
        }
    }
}
