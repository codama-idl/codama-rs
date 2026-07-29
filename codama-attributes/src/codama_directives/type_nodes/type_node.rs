use crate::{utils::FromMeta, TypeDirectiveNode};
use codama_nodes::{
    AmountTypeNode, ArrayTypeNode, BooleanTypeNode, BytesTypeNode, DateTimeTypeNode,
    FixedSizeTypeNode, HiddenPrefixTypeNode, HiddenSuffixTypeNode, MapTypeNode, NumberTypeNode,
    OptionTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, PublicKeyTypeNode, RegisteredTypeNode,
    RemainderOptionTypeNode, SentinelTypeNode, SetTypeNode, SizePrefixTypeNode, SolAmountTypeNode,
    StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, TypeNode,
    ZeroableOptionTypeNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for RegisteredTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "amount" => AmountTypeNode::from_meta(meta).map(Self::from),
            "array" => ArrayTypeNode::from_meta(meta).map(Self::from),
            "boolean" => BooleanTypeNode::from_meta(meta).map(Self::from),
            "bytes" => BytesTypeNode::from_meta(meta).map(Self::from),
            "date_time" => DateTimeTypeNode::from_meta(meta).map(Self::from),
            "field" => StructFieldTypeNode::from_meta(meta).map(Self::from),
            "fixed_size" => FixedSizeTypeNode::from_meta(meta).map(Self::from),
            "hidden_prefix" => HiddenPrefixTypeNode::from_meta(meta).map(Self::from),
            "hidden_suffix" => HiddenSuffixTypeNode::from_meta(meta).map(Self::from),
            "map" => MapTypeNode::from_meta(meta).map(Self::from),
            "number" => NumberTypeNode::from_meta(meta).map(Self::from),
            "option" => OptionTypeNode::from_meta(meta).map(Self::from),
            "post_offset" => PostOffsetTypeNode::from_meta(meta).map(Self::from),
            "pre_offset" => PreOffsetTypeNode::from_meta(meta).map(Self::from),
            "public_key" => PublicKeyTypeNode::from_meta(meta).map(Self::from),
            "remainder_option" => RemainderOptionTypeNode::from_meta(meta).map(Self::from),
            "sentinel" => SentinelTypeNode::from_meta(meta).map(Self::from),
            "set" => SetTypeNode::from_meta(meta).map(Self::from),
            "size_prefix" => SizePrefixTypeNode::from_meta(meta).map(Self::from),
            "sol_amount" => SolAmountTypeNode::from_meta(meta).map(Self::from),
            "string" => StringTypeNode::from_meta(meta).map(Self::from),
            "struct" => StructTypeNode::from_meta(meta).map(Self::from),
            "tuple" => TupleTypeNode::from_meta(meta).map(Self::from),
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
