use crate::TypeNode;
use codama_nodes_derive::{node, TypeNode};

#[node]
#[derive(TypeNode)]
pub struct RemainderOptionTypeNode {
    // Children.
    pub item: TypeNode,
}

impl RemainderOptionTypeNode {
    pub fn new<T>(item: T) -> Self
    where
        T: Into<TypeNode>,
    {
        Self { item: item.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, U64};

    #[test]
    fn new() {
        let node = RemainderOptionTypeNode::new(NumberTypeNode::le(U64));
        assert_eq!(node.item, TypeNode::Number(NumberTypeNode::le(U64)));
    }
}
