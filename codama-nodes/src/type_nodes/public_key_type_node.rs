use codama_nodes_derive::{node, TypeNode};

#[node]
#[derive(TypeNode)]
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
