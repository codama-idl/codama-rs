use codama_nodes_derive::node;

#[node]
pub struct IdentityValueNode {}

impl Into<crate::Node> for IdentityValueNode {
    fn into(self) -> crate::Node {
        crate::Node::ContextualValue(self.into())
    }
}

impl IdentityValueNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = IdentityValueNode::new();
        assert_eq!(node, IdentityValueNode {});
    }

    #[test]
    fn to_json() {
        let node = IdentityValueNode::new();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"identityValueNode"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"identityValueNode"}"#;
        let node: IdentityValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, IdentityValueNode::new());
    }
}
