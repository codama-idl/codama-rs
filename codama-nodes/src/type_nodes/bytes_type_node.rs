use codama_nodes_derive::type_node;

#[type_node]
pub struct BytesTypeNode {}

impl BytesTypeNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = BytesTypeNode::new();
        assert_eq!(node, BytesTypeNode {});
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
