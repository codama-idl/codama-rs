use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq, Clone)]
pub struct SizeDiscriminatorNode {
    // Data.
    pub size: usize,
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
}
