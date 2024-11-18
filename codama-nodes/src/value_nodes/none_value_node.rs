use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct NoneValueNode {}

impl NoneValueNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = NoneValueNode::new();
        assert_eq!(node, NoneValueNode {});
    }
}
