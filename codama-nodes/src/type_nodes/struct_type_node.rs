use crate::StructFieldTypeNode;
use codama_nodes_derive::{node, TypeNode};

#[node]
#[derive(TypeNode)]
pub struct StructTypeNode {
    // Children.
    pub fields: Vec<StructFieldTypeNode>,
}

impl StructTypeNode {
    pub fn new(fields: Vec<StructFieldTypeNode>) -> Self {
        Self { fields }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, StringTypeNode, U32};

    #[test]
    fn new() {
        let node = StructTypeNode::new(vec![
            StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
        ]);
        assert_eq!(
            node.fields,
            vec![
                StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            ]
        );
    }
}
