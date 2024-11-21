use codama_nodes_derive::{node, TypeNode};

#[node]
#[derive(TypeNode)]
pub struct BytesTypeNode {}

impl BytesTypeNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = BytesTypeNode::new();
        assert_eq!(node, BytesTypeNode {});
    }
}
