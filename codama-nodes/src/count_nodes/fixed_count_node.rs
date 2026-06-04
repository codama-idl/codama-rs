use crate::FixedCountNode;

impl FixedCountNode {
    pub fn new(value: u64) -> Self {
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
