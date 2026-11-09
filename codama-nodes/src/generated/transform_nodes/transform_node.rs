use crate::{
    FixedSizeTransformNode, HiddenPrefixTransformNode, HiddenSuffixTransformNode,
    PostOffsetTransformNode, PreOffsetTransformNode, SentinelTransformNode,
    SizePrefixTransformNode,
};
use codama_nodes_derive::{node_union, NodeConversions};

#[derive(NodeConversions)]
#[node_union]
pub enum TransformNode {
    FixedSize(FixedSizeTransformNode),
    HiddenPrefix(HiddenPrefixTransformNode),
    HiddenSuffix(HiddenSuffixTransformNode),
    PostOffset(PostOffsetTransformNode),
    PreOffset(PreOffsetTransformNode),
    Sentinel(SentinelTransformNode),
    SizePrefix(SizePrefixTransformNode),
}
