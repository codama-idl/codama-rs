use codama_nodes_derive::node;

#[node]
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
