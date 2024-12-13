use codama_nodes_derive::node;

#[node]
pub struct SizeDiscriminatorNode {
    // Data.
    pub size: usize,
}

impl From<SizeDiscriminatorNode> for crate::Node {
    fn from(val: SizeDiscriminatorNode) -> Self {
        crate::Node::Discriminator(val.into())
    }
}

impl SizeDiscriminatorNode {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = SizeDiscriminatorNode::new(42);
        assert_eq!(node.size, 42);
    }

    #[test]
    fn to_json() {
        let node = SizeDiscriminatorNode::new(42);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"sizeDiscriminatorNode","size":42}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"sizeDiscriminatorNode","size":42}"#;
        let node: SizeDiscriminatorNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, SizeDiscriminatorNode::new(42));
    }
}
