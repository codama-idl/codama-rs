use crate::{ConstantValueNode, SentinelTransformNode};

impl SentinelTransformNode {
    pub fn new<T>(sentinel: T) -> Self
    where
        T: Into<ConstantValueNode>,
    {
        Self {
            sentinel: sentinel.into(),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Base16;

    #[test]
    fn new() {
        let node = SentinelTransformNode::new(ConstantValueNode::bytes(Base16, "ff"));
        assert_eq!(node.sentinel, ConstantValueNode::bytes(Base16, "ff"));
    }

    #[test]
    fn to_json() {
        let node = SentinelTransformNode::new(ConstantValueNode::bytes(Base16, "ff"));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"sentinelTransformNode","sentinel":{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ff","encoding":"base16"}}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"sentinelTransformNode","sentinel":{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ff","encoding":"base16"}}}"#;
        let node: SentinelTransformNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            SentinelTransformNode::new(ConstantValueNode::bytes(Base16, "ff"))
        );
    }
}
