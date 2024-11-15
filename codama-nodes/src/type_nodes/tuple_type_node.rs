use codama_nodes_derive::{Node, TypeNode};

use super::TypeNode;

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct TupleTypeNode {
    // Children.
    pub items: Vec<TypeNode>,
}

impl TupleTypeNode {
    pub fn new(items: Vec<TypeNode>) -> Self {
        Self { items }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, StringTypeNode, U32};

    #[test]
    fn new() {
        let node = TupleTypeNode::new(vec![
            NumberTypeNode::le(U32).into(),
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
