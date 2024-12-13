use codama_nodes_derive::node;

#[node]
pub struct BooleanValueNode {
    // Data.
    pub boolean: bool,
}

impl From<BooleanValueNode> for crate::Node {
    fn from(val: BooleanValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}

impl BooleanValueNode {
    pub fn new(boolean: bool) -> Self {
        Self { boolean }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert!(BooleanValueNode::new(true).boolean);
        assert!(!BooleanValueNode::new(false).boolean);
    }

    #[test]
    fn to_json() {
        let node = BooleanValueNode::new(true);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"booleanValueNode","boolean":true}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"booleanValueNode","boolean":true}"#;
        let node: BooleanValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, BooleanValueNode::new(true));
    }
}
