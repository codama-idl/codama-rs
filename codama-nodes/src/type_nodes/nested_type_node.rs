use crate::{
    FixedSizeTypeNode, HiddenPrefixTypeNode, HiddenSuffixTypeNode, NestedTypeNodeTrait, Node,
    NodeTrait, NodeUnionTrait, PostOffsetTypeNode, PreOffsetTypeNode, SentinelTypeNode,
    SizePrefixTypeNode, TypeNode, TypeNodeTrait, TypeNodeUnionTrait,
};
use codama_errors::CodamaError;
use codama_nodes_derive::node_union;

#[node_union]
pub enum NestedTypeNode<T: TypeNodeTrait> {
    FixedSize(Box<FixedSizeTypeNode<NestedTypeNode<T>>>),
    HiddenPrefix(Box<HiddenPrefixTypeNode<NestedTypeNode<T>>>),
    HiddenSuffix(Box<HiddenSuffixTypeNode<NestedTypeNode<T>>>),
    PostOffset(Box<PostOffsetTypeNode<NestedTypeNode<T>>>),
    PreOffset(Box<PreOffsetTypeNode<NestedTypeNode<T>>>),
    Sentinel(Box<SentinelTypeNode<NestedTypeNode<T>>>),
    SizePrefix(Box<SizePrefixTypeNode<NestedTypeNode<T>>>),
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
            NestedTypeNode::FixedSize(node) => {
                NestedTypeNode::FixedSize(Box::new(node.map_nested_type_node(f)))
            }
            NestedTypeNode::HiddenPrefix(node) => {
                NestedTypeNode::HiddenPrefix(Box::new(node.map_nested_type_node(f)))
            }
            NestedTypeNode::HiddenSuffix(node) => {
                NestedTypeNode::HiddenSuffix(Box::new(node.map_nested_type_node(f)))
            }
            NestedTypeNode::PostOffset(node) => {
                NestedTypeNode::PostOffset(Box::new(node.map_nested_type_node(f)))
            }
            NestedTypeNode::PreOffset(node) => {
                NestedTypeNode::PreOffset(Box::new(node.map_nested_type_node(f)))
            }
            NestedTypeNode::Sentinel(node) => {
                NestedTypeNode::Sentinel(Box::new(node.map_nested_type_node(f)))
            }
            NestedTypeNode::SizePrefix(node) => {
                NestedTypeNode::SizePrefix(Box::new(node.map_nested_type_node(f)))
            }
            NestedTypeNode::Value(value) => NestedTypeNode::Value(f(value)),
        }
    }
}

impl<T: TypeNodeTrait> TryFrom<Node> for NestedTypeNode<T> {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        TypeNode::try_from(node)?.try_into()
    }
}

impl<T: TypeNodeTrait> TryFrom<TypeNode> for NestedTypeNode<T> {
    type Error = CodamaError;

    fn try_from(node: TypeNode) -> Result<Self, Self::Error> {
        match node {
            TypeNode::FixedSize(node) => {
                Ok(NestedTypeNode::FixedSize(Box::new(FixedSizeTypeNode {
                    size: node.size,
                    r#type: node.r#type.try_into()?,
                })))
            }
            TypeNode::HiddenPrefix(node) => Ok(NestedTypeNode::HiddenPrefix(Box::new(
                HiddenPrefixTypeNode {
                    r#type: node.r#type.try_into()?,
                    prefix: node.prefix,
                },
            ))),
            TypeNode::HiddenSuffix(node) => Ok(NestedTypeNode::HiddenSuffix(Box::new(
                HiddenSuffixTypeNode {
                    r#type: node.r#type.try_into()?,
                    suffix: node.suffix,
                },
            ))),
            TypeNode::PostOffset(node) => {
                Ok(NestedTypeNode::PostOffset(Box::new(PostOffsetTypeNode {
                    offset: node.offset,
                    strategy: node.strategy,
                    r#type: node.r#type.try_into()?,
                })))
            }
            TypeNode::PreOffset(node) => {
                Ok(NestedTypeNode::PreOffset(Box::new(PreOffsetTypeNode {
                    offset: node.offset,
                    strategy: node.strategy,
                    r#type: node.r#type.try_into()?,
                })))
            }
            TypeNode::Sentinel(node) => Ok(NestedTypeNode::Sentinel(Box::new(SentinelTypeNode {
                r#type: node.r#type.try_into()?,
                sentinel: node.sentinel,
            }))),
            TypeNode::SizePrefix(node) => {
                Ok(NestedTypeNode::SizePrefix(Box::new(SizePrefixTypeNode {
                    r#type: node.r#type.try_into()?,
                    prefix: node.prefix,
                })))
            }
            _ => Ok(NestedTypeNode::Value(T::from_type_node(node)?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NodeUnionTrait, StringTypeNode};

    #[test]
    fn kind() {
        let node: NestedTypeNode<StringTypeNode> = StringTypeNode::utf8().into();
        assert_eq!(node.kind(), "stringTypeNode");
    }
}
