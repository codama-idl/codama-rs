use codama_nodes_derive::node;

#[node]
pub struct ProgramIdValueNode {}

impl Into<crate::Node> for ProgramIdValueNode {
    fn into(self) -> crate::Node {
        crate::Node::ContextualValue(self.into())
    }
}

impl ProgramIdValueNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = ProgramIdValueNode::new();
        assert_eq!(node, ProgramIdValueNode {});
    }

    #[test]
    fn to_json() {
        let node = ProgramIdValueNode::new();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"programIdValueNode"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"programIdValueNode"}"#;
        let node: ProgramIdValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ProgramIdValueNode::new());
    }
}
