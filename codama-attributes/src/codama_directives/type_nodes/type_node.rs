use crate::utils::FromMeta;
use codama_nodes::{
    BooleanTypeNode, FixedSizeTypeNode, NumberTypeNode, PublicKeyTypeNode, RegisteredTypeNode,
    StringTypeNode, StructFieldTypeNode, StructTypeNode, TypeNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for RegisteredTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "boolean" => BooleanTypeNode::from_meta(meta).map(Self::from),
            "field" => StructFieldTypeNode::from_meta(meta).map(Self::from),
            "fixed_size" => FixedSizeTypeNode::from_meta(meta).map(Self::from),
            "number" => NumberTypeNode::from_meta(meta).map(Self::from),
            "public_key" => PublicKeyTypeNode::from_meta(meta).map(Self::from),
            "string" => StringTypeNode::from_meta(meta).map(Self::from),
            "struct" => StructTypeNode::from_meta(meta).map(Self::from),
            _ => Err(meta.error("unrecognized type")),
        }
    }
}

impl FromMeta for TypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        Self::try_from(RegisteredTypeNode::from_meta(meta)?)
            .map_err(|_| meta.error("unrecognized type"))
    }
}
