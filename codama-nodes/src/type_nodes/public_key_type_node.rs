use codama_nodes_derive::type_node;

#[type_node]
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
