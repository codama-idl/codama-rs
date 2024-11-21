use crate::{
    AmountTypeNode, ArrayTypeNode, BooleanTypeNode, BytesTypeNode, DateTimeTypeNode, EnumTypeNode,
    FixedSizeTypeNode, HiddenPrefixTypeNode, HiddenSuffixTypeNode, MapTypeNode, NumberTypeNode,
    OptionTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, PublicKeyTypeNode,
    RemainderOptionTypeNode, SentinelTypeNode, SetTypeNode, SizePrefixTypeNode, SolAmountTypeNode,
    StringTypeNode, StructTypeNode, TupleTypeNode, TypeNodeEnumTrait, ZeroableOptionTypeNode,
};
use serde::{Deserialize, Serialize};

#[derive(
    codama_nodes_derive::IntoEnum, core::fmt::Debug, core::cmp::PartialEq, core::clone::Clone,
)]
pub enum TypeNode {
    Amount(AmountTypeNode),
    Array(Box<ArrayTypeNode>),
    Boolean(BooleanTypeNode),
    Bytes(BytesTypeNode),
    DateTime(DateTimeTypeNode),
    Enum(EnumTypeNode),
    FixedSize(Box<FixedSizeTypeNode<TypeNode>>),
    HiddenPrefix(Box<HiddenPrefixTypeNode<TypeNode>>),
    HiddenSuffix(Box<HiddenSuffixTypeNode<TypeNode>>),
    Map(Box<MapTypeNode>),
    Number(NumberTypeNode),
    Option(Box<OptionTypeNode>),
    PostOffset(Box<PostOffsetTypeNode<TypeNode>>),
    PreOffset(Box<PreOffsetTypeNode<TypeNode>>),
    PublicKey(PublicKeyTypeNode),
    RemainderOption(Box<RemainderOptionTypeNode>),
    Sentinel(Box<SentinelTypeNode<TypeNode>>),
    Set(Box<SetTypeNode>),
    SizePrefix(Box<SizePrefixTypeNode<TypeNode>>),
    SolAmount(SolAmountTypeNode),
    String(StringTypeNode),
    Struct(StructTypeNode),
    Tuple(TupleTypeNode),
    ZeroableOption(Box<ZeroableOptionTypeNode>),
}

impl TypeNodeEnumTrait for TypeNode {}

impl Serialize for TypeNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TypeNode::Amount(node) => node.serialize(serializer),
            TypeNode::Array(node) => node.serialize(serializer),
            TypeNode::Boolean(node) => node.serialize(serializer),
            TypeNode::Bytes(node) => node.serialize(serializer),
            TypeNode::DateTime(node) => node.serialize(serializer),
            TypeNode::Enum(node) => node.serialize(serializer),
            TypeNode::FixedSize(node) => node.serialize(serializer),
            TypeNode::HiddenPrefix(node) => node.serialize(serializer),
            TypeNode::HiddenSuffix(node) => node.serialize(serializer),
            TypeNode::Map(node) => node.serialize(serializer),
            TypeNode::Number(node) => node.serialize(serializer),
            TypeNode::Option(node) => node.serialize(serializer),
            TypeNode::PostOffset(node) => node.serialize(serializer),
            TypeNode::PreOffset(node) => node.serialize(serializer),
            TypeNode::PublicKey(node) => node.serialize(serializer),
            TypeNode::RemainderOption(node) => node.serialize(serializer),
            TypeNode::Sentinel(node) => node.serialize(serializer),
            TypeNode::Set(node) => node.serialize(serializer),
            TypeNode::SizePrefix(node) => node.serialize(serializer),
            TypeNode::SolAmount(node) => node.serialize(serializer),
            TypeNode::String(node) => node.serialize(serializer),
            TypeNode::Struct(node) => node.serialize(serializer),
            TypeNode::Tuple(node) => node.serialize(serializer),
            TypeNode::ZeroableOption(node) => node.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for TypeNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let kind = value["kind"]
            .as_str()
            .ok_or_else(|| serde::de::Error::custom("missing kind"))?;
        let to_serde_error = |e: serde_json::Error| -> D::Error {
            serde::de::Error::custom(format!("failed to deserialize AmountTypeNode: {}", e))
        };
        match kind {
            "amountTypeNode" => Ok(TypeNode::Amount(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "arrayTypeNode" => Ok(TypeNode::Array(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "booleanTypeNode" => Ok(TypeNode::Boolean(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "bytesTypeNode" => Ok(TypeNode::Bytes(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "dateTimeTypeNode" => Ok(TypeNode::DateTime(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "enumTypeNode" => Ok(TypeNode::Enum(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "fixedSizeTypeNode" => Ok(TypeNode::FixedSize(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "hiddenPrefixTypeNode" => Ok(TypeNode::HiddenPrefix(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "hiddenSuffixTypeNode" => Ok(TypeNode::HiddenSuffix(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "mapTypeNode" => Ok(TypeNode::Map(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "numberTypeNode" => Ok(TypeNode::Number(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "optionTypeNode" => Ok(TypeNode::Option(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "postOffsetTypeNode" => Ok(TypeNode::PostOffset(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "preOffsetTypeNode" => Ok(TypeNode::PreOffset(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "publicKeyTypeNode" => Ok(TypeNode::PublicKey(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "remainderOptionTypeNode" => Ok(TypeNode::RemainderOption(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "sentinelTypeNode" => Ok(TypeNode::Sentinel(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "setTypeNode" => Ok(TypeNode::Set(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "sizePrefixTypeNode" => Ok(TypeNode::SizePrefix(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "solAmountTypeNode" => Ok(TypeNode::SolAmount(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "stringTypeNode" => Ok(TypeNode::String(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "structTypeNode" => Ok(TypeNode::Struct(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "tupleTypeNode" => Ok(TypeNode::Tuple(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            "zeroableOptionTypeNode" => Ok(TypeNode::ZeroableOption(
                serde_json::from_value(value).map_err(to_serde_error)?,
            )),
            _ => Err(serde::de::Error::custom("unknown kind for TypeNode")),
        }
    }
}
