use codama_nodes_derive::node;

#[node]
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
