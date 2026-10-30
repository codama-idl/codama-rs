use crate::FixedSizeTransformNode;

impl FixedSizeTransformNode {
    pub fn new(size: u64) -> Self {
        Self {
            size,
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = FixedSizeTransformNode::new(42);
        assert_eq!(node.size, 42);
    }

    #[test]
    fn to_json() {
        let node = FixedSizeTransformNode::new(42);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"fixedSizeTransformNode","size":42}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"fixedSizeTransformNode","size":42}"#;
        let node: FixedSizeTransformNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, FixedSizeTransformNode::new(42));
    }
}
