use codama_nodes_derive::{Node, TypeNode};

use super::TypeNode;

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct TupleTypeNode {
    // Children.
    pub items: Vec<TypeNode>,
}

impl TupleTypeNode {
    pub fn new<T>(items: T) -> Self
    where
        T: IntoIterator<Item = TypeNode>,
    {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, StringTypeNode, U32};

    #[test]
    fn new() {
        let node = TupleTypeNode::new(vec![
            NumberTypeNode::le(U32).into(), // TODO: try to improve the API here.
            StringTypeNode::utf8().into(),
        ]);
        assert_eq!(
            node.items,
            vec![
                TypeNode::Number(NumberTypeNode::le(U32)),
                TypeNode::String(StringTypeNode::utf8())
            ]
        );
    }
}
