use codama_nodes_derive::node;

#[node]
pub struct PayerValueNode {}

impl Into<crate::Node> for PayerValueNode {
    fn into(self) -> crate::Node {
        crate::Node::ContextualValue(self.into())
    }
}

impl PayerValueNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = PayerValueNode::new();
        assert_eq!(node, PayerValueNode {});
    }

    #[test]
    fn to_json() {
        let node = PayerValueNode::new();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"payerValueNode"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"payerValueNode"}"#;
        let node: PayerValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PayerValueNode::new());
    }
}
