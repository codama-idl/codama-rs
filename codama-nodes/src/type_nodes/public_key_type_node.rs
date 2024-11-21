use codama_nodes_derive::type_node;

#[type_node]
pub struct PublicKeyTypeNode {}

impl PublicKeyTypeNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = PublicKeyTypeNode::new();
        assert_eq!(node, PublicKeyTypeNode {});
    }

    #[test]
    fn to_json() {
        let node = PublicKeyTypeNode::new();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"publicKeyTypeNode"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"publicKeyTypeNode"}"#;
        let node: PublicKeyTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PublicKeyTypeNode::new());
    }
}
