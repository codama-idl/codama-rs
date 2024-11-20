use crate::{
    FixedSizeTypeNode, HiddenPrefixTypeNode, HiddenSuffixTypeNode, NestedTypeNodeTrait,
    PostOffsetTypeNode, PreOffsetTypeNode, SentinelTypeNode, SizePrefixTypeNode, TypeNodeEnumTrait,
    TypeNodeTrait,
};
use codama_nodes_derive::IntoEnum;
use serde::{Deserialize, Serialize};

#[derive(IntoEnum, Debug, PartialEq, Clone)]
pub enum NestedTypeNode<T: TypeNodeTrait> {
    FixedSize(Box<FixedSizeTypeNode<NestedTypeNode<T>>>),
    HiddenPrefix(Box<HiddenPrefixTypeNode<NestedTypeNode<T>>>),
    HiddenSuffix(Box<HiddenSuffixTypeNode<NestedTypeNode<T>>>),
    PostOffset(Box<PostOffsetTypeNode<NestedTypeNode<T>>>),
    PreOffset(Box<PreOffsetTypeNode<NestedTypeNode<T>>>),
    Sentinel(Box<SentinelTypeNode<NestedTypeNode<T>>>),
    SizePrefix(Box<SizePrefixTypeNode<NestedTypeNode<T>>>),
    Value(T),
}

impl<T: TypeNodeTrait> TypeNodeEnumTrait for NestedTypeNode<T> {}

impl<T: TypeNodeTrait> NestedTypeNodeTrait<T> for NestedTypeNode<T> {
    fn get_nested_type_node(&self) -> &T {
        match self {
            NestedTypeNode::FixedSize(node) => node.get_nested_type_node(),
            NestedTypeNode::HiddenPrefix(node) => node.get_nested_type_node(),
            NestedTypeNode::HiddenSuffix(node) => node.get_nested_type_node(),
            NestedTypeNode::PostOffset(node) => node.get_nested_type_node(),
            NestedTypeNode::PreOffset(node) => node.get_nested_type_node(),
            NestedTypeNode::Sentinel(node) => node.get_nested_type_node(),
            NestedTypeNode::SizePrefix(node) => node.get_nested_type_node(),
            NestedTypeNode::Value(value) => value,
        }
    }
}

// Implement Serialize to add the "kind" field.
impl<T> Serialize for NestedTypeNode<T>
where
    T: TypeNodeTrait + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let NestedTypeNode::Value(value) = self else {
            return Err(serde::ser::Error::custom("TODO"));
        };
        value.serialize(serializer)
    }
}

// Implement Deserialize to handle the "kind" field.
impl<'de, T> Deserialize<'de> for NestedTypeNode<T>
where
    T: TypeNodeTrait + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(NestedTypeNode::Value)
    }
}
