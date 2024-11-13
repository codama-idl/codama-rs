use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct FixedCountNode {
    // Data.
    pub value: usize,
}

impl FixedCountNode {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = FixedCountNode::new(42);
        assert_eq!(node.value, 42);
    }
}
