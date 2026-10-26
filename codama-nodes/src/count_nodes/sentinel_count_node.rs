use crate::{ConstantValueNode, SentinelCountNode};

impl SentinelCountNode {
    pub fn new<T>(sentinel: T) -> Self
    where
        T: Into<ConstantValueNode>,
    {
        Self {
            strategy: None,
            sentinel: sentinel.into(),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base16, SentinelCountStrategy};

    #[test]
    fn new() {
        let node = SentinelCountNode::new(ConstantValueNode::bytes(Base16, "ff"));
        assert_eq!(node.strategy, None);
        assert_eq!(node.sentinel, ConstantValueNode::bytes(Base16, "ff"));
    }

    #[test]
    fn to_json() {
        let node = SentinelCountNode {
            strategy: Some(SentinelCountStrategy::Optional),
            ..SentinelCountNode::new(ConstantValueNode::bytes(Base16, "ff"))
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"sentinelCountNode","strategy":"optional","sentinel":{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ff","encoding":"base16"}}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"sentinelCountNode","sentinel":{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ff","encoding":"base16"}}}"#;
        let node: SentinelCountNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            SentinelCountNode::new(ConstantValueNode::bytes(Base16, "ff"))
        );
    }
}
