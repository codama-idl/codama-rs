use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq)]
pub struct ProgramIdValueNode {}

impl ProgramIdValueNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = ProgramIdValueNode::new();
        assert_eq!(node, ProgramIdValueNode {});
    }
}
