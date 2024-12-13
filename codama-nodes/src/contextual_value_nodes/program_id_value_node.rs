use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct ProgramIdValueNode {}

impl From<ProgramIdValueNode> for crate::Node {
    fn from(val: ProgramIdValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl ProgramIdValueNode {
    pub fn new() -> Self {
        Self::default()
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
