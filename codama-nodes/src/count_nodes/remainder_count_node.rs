use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct RemainderCountNode {}

impl From<RemainderCountNode> for crate::Node {
    fn from(val: RemainderCountNode) -> Self {
        crate::Node::Count(val.into())
    }
}

impl RemainderCountNode {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = RemainderCountNode::new();
        assert_eq!(node, RemainderCountNode {});
    }

    #[test]
    fn to_json() {
        let node = RemainderCountNode::new();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"remainderCountNode"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"remainderCountNode"}"#;
        let node: RemainderCountNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, RemainderCountNode::new());
    }
}
