use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq, Clone)]
pub struct BooleanValueNode {
    // Data.
    pub boolean: bool,
}

impl BooleanValueNode {
    pub fn new(boolean: bool) -> Self {
        Self { boolean }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(BooleanValueNode::new(true).boolean, true);
        assert_eq!(BooleanValueNode::new(false).boolean, false);
    }
}
