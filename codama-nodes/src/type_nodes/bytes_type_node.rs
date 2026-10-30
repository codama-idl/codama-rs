use crate::BytesTypeNode;

impl BytesTypeNode {
    pub fn new() -> Self {
        Self {
            transforms: vec![],
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = BytesTypeNode::new();
        assert_eq!(
            node,
            BytesTypeNode {
                transforms: vec![],
                plugins: vec![],
            }
        );
    }

    #[test]
    fn to_json() {
        let node = BytesTypeNode::new();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"bytesTypeNode"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"bytesTypeNode"}"#;
        let node: BytesTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, BytesTypeNode::new());
    }
}
