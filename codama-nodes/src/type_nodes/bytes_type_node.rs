use codama_nodes_derive::type_node;

#[type_node]
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
