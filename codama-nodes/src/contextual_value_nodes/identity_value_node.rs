use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct IdentityValueNode {}

impl From<IdentityValueNode> for crate::Node {
    fn from(val: IdentityValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl IdentityValueNode {
    pub fn new() -> Self {
        Self::default()
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
