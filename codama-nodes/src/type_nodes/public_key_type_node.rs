use codama_nodes_derive::{Node, TypeNode};

#[derive(Node, TypeNode, Debug, PartialEq)]
pub struct PublicKeyTypeNode {}

impl PublicKeyTypeNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = PublicKeyTypeNode::new();
        assert_eq!(node, PublicKeyTypeNode {});
    }
}
