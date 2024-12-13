use codama_nodes_derive::node;

#[node]
pub struct FixedCountNode {
    // Data.
    pub value: usize,
}

impl From<FixedCountNode> for crate::Node {
    fn from(val: FixedCountNode) -> Self {
        crate::Node::Count(val.into())
    }
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

    #[test]
    fn to_json() {
        let node = FixedCountNode::new(42);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"fixedCountNode","value":42}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"fixedCountNode","value":42}"#;
        let node: FixedCountNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, FixedCountNode::new(42));
    }
}
