use crate::{utils::FromMeta, TypeDirectiveNode};
use codama_nodes::{
    BooleanTypeNode, BytesTypeNode, FixedSizeTypeNode, NumberTypeNode, OptionTypeNode,
    PublicKeyTypeNode, RegisteredTypeNode, StringTypeNode, StructFieldTypeNode, StructTypeNode,
    TypeNode, ZeroableOptionTypeNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for RegisteredTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "boolean" => BooleanTypeNode::from_meta(meta).map(Self::from),
            "bytes" => BytesTypeNode::from_meta(meta).map(Self::from),
            "field" => StructFieldTypeNode::from_meta(meta).map(Self::from),
            "fixed_size" => FixedSizeTypeNode::from_meta(meta).map(Self::from),
            "number" => NumberTypeNode::from_meta(meta).map(Self::from),
            "option" => OptionTypeNode::from_meta(meta).map(Self::from),
            "public_key" => PublicKeyTypeNode::from_meta(meta).map(Self::from),
            "string" => StringTypeNode::from_meta(meta).map(Self::from),
            "struct" => StructTypeNode::from_meta(meta).map(Self::from),
            "zeroable_option" => ZeroableOptionTypeNode::from_meta(meta).map(Self::from),
            _ => Err(meta.error("unrecognized type")),
        }
    }
}

impl FromMeta for TypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        Self::try_from(TypeDirectiveNode::from_meta(meta)?)
            .map_err(|_| meta.error("unrecognized type"))
    }
}
