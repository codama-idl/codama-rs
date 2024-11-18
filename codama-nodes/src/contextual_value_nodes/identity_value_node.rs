use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct IdentityValueNode {}

impl IdentityValueNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = IdentityValueNode::new();
        assert_eq!(node, IdentityValueNode {});
    }
}
