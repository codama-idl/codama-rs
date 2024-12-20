use crate::{
    FixedSizeTypeNode, HasKind, HiddenPrefixTypeNode, HiddenSuffixTypeNode, NestedTypeNodeTrait,
    Node, NodeUnionTrait, PostOffsetTypeNode, PreOffsetTypeNode, SentinelTypeNode,
    SizePrefixTypeNode, TypeNode, TypeNodeTrait, TypeNodeUnionTrait,
};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes_derive::node_union;

#[node_union]
pub enum NestedTypeNode<T: TypeNodeTrait> {
    FixedSize(FixedSizeTypeNode<NestedTypeNode<T>>),
    HiddenPrefix(HiddenPrefixTypeNode<NestedTypeNode<T>>),
    HiddenSuffix(HiddenSuffixTypeNode<NestedTypeNode<T>>),
    PostOffset(PostOffsetTypeNode<NestedTypeNode<T>>),
    PreOffset(PreOffsetTypeNode<NestedTypeNode<T>>),
    Sentinel(SentinelTypeNode<NestedTypeNode<T>>),
    SizePrefix(SizePrefixTypeNode<NestedTypeNode<T>>),
    #[fallback]
    Value(T),
}

impl<T: TypeNodeTrait> TypeNodeUnionTrait for NestedTypeNode<T> {}

impl<T: TypeNodeTrait> NestedTypeNodeTrait<T> for NestedTypeNode<T> {
    type Mapped<U: TypeNodeTrait> = NestedTypeNode<U>;

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

    fn map_nested_type_node<U: TypeNodeTrait, F: FnOnce(T) -> U>(self, f: F) -> Self::Mapped<U> {
        match self {
            Self::FixedSize(node) => Self::Mapped::FixedSize(node.map_nested_type_node(f)),
            Self::HiddenPrefix(node) => Self::Mapped::HiddenPrefix(node.map_nested_type_node(f)),
            Self::HiddenSuffix(node) => Self::Mapped::HiddenSuffix(node.map_nested_type_node(f)),
            Self::PostOffset(node) => Self::Mapped::PostOffset(node.map_nested_type_node(f)),
            Self::PreOffset(node) => Self::Mapped::PreOffset(node.map_nested_type_node(f)),
            Self::Sentinel(node) => Self::Mapped::Sentinel(node.map_nested_type_node(f)),
            Self::SizePrefix(node) => Self::Mapped::SizePrefix(node.map_nested_type_node(f)),
            Self::Value(value) => Self::Mapped::Value(f(value)),
        }
    }
}

impl<T: TypeNodeTrait> TryFrom<Node> for NestedTypeNode<T> {
    type Error = CodamaError;

    fn try_from(node: Node) -> CodamaResult<Self> {
        TypeNode::try_from(node)?.try_into()
    }
}

impl<T: TypeNodeTrait> TryFrom<TypeNode> for NestedTypeNode<T> {
    type Error = CodamaError;

    fn try_from(node: TypeNode) -> CodamaResult<Self> {
        match node {
            TypeNode::FixedSize(node) => Ok(NestedTypeNode::FixedSize(node.try_into()?)),
            TypeNode::HiddenPrefix(node) => Ok(NestedTypeNode::HiddenPrefix(node.try_into()?)),
            TypeNode::HiddenSuffix(node) => Ok(NestedTypeNode::HiddenSuffix(node.try_into()?)),
            TypeNode::PostOffset(node) => Ok(NestedTypeNode::PostOffset(node.try_into()?)),
            TypeNode::PreOffset(node) => Ok(NestedTypeNode::PreOffset(node.try_into()?)),
            TypeNode::Sentinel(node) => Ok(NestedTypeNode::Sentinel(node.try_into()?)),
            TypeNode::SizePrefix(node) => Ok(NestedTypeNode::SizePrefix(node.try_into()?)),
            _ => Ok(NestedTypeNode::Value(T::try_from_type_node(node)?)),
        }
    }
}

impl<T: TypeNodeTrait> TryFrom<NestedTypeNode<T>> for Node {
    type Error = CodamaError;

    fn try_from(node: NestedTypeNode<T>) -> CodamaResult<Self> {
        let type_node: TypeNode = node.into();
        Ok(Node::Type(type_node.try_into()?))
    }
}

impl<T: TypeNodeTrait> From<NestedTypeNode<T>> for TypeNode {
    fn from(node: NestedTypeNode<T>) -> Self {
        match node {
            NestedTypeNode::FixedSize(node) => node.into_type_node(),
            NestedTypeNode::HiddenPrefix(node) => node.into_type_node(),
            NestedTypeNode::HiddenSuffix(node) => node.into_type_node(),
            NestedTypeNode::PostOffset(node) => node.into_type_node(),
            NestedTypeNode::PreOffset(node) => node.into_type_node(),
            NestedTypeNode::Sentinel(node) => node.into_type_node(),
            NestedTypeNode::SizePrefix(node) => node.into_type_node(),
            NestedTypeNode::Value(value) => T::into_type_node(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StringTypeNode;

    #[test]
    fn kind() {
        let node: NestedTypeNode<StringTypeNode> = StringTypeNode::utf8().into();
        assert_eq!(node.kind(), "stringTypeNode");
    }

    #[test]
    fn from_type_node() {
        let node: TypeNode = FixedSizeTypeNode::<TypeNode>::new(StringTypeNode::utf8(), 42).into();
        let node: NestedTypeNode<StringTypeNode> = node.try_into().unwrap();
        assert_eq!(
            node,
            NestedTypeNode::FixedSize(FixedSizeTypeNode::new(StringTypeNode::utf8(), 42))
        );
    }

    #[test]
    fn into_type_node() {
        let node = NestedTypeNode::FixedSize(FixedSizeTypeNode::new(StringTypeNode::utf8(), 42));
        let node: TypeNode = node.try_into().unwrap();
        assert_eq!(
            node,
            TypeNode::FixedSize(FixedSizeTypeNode::new(StringTypeNode::utf8(), 42))
        );
    }
}
