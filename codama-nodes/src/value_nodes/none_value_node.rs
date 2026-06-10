use crate::NoneValueNode;

impl NoneValueNode {
    pub fn new() -> Self {
        Self::default()
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

    #[test]
    fn to_json() {
        let node = NoneValueNode::new();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"noneValueNode"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"noneValueNode"}"#;
        let node: NoneValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, NoneValueNode::new());
    }
}
