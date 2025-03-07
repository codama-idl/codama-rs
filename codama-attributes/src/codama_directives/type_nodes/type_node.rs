use crate::utils::FromMeta;
use codama_nodes::{
    BooleanTypeNode, FixedSizeTypeNode, NumberTypeNode, PublicKeyTypeNode, StringTypeNode, TypeNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for TypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "boolean" => BooleanTypeNode::from_meta(meta).map(Self::from),
            "fixed_size" => FixedSizeTypeNode::from_meta(meta).map(Self::from),
            "number" => NumberTypeNode::from_meta(meta).map(Self::from),
            "public_key" => PublicKeyTypeNode::from_meta(meta).map(Self::from),
            "string" => StringTypeNode::from_meta(meta).map(Self::from),
            _ => Err(meta.error("unrecognized type")),
        }
    }
}
