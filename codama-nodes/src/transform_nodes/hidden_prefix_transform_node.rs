use crate::{ConstantValueNode, HiddenPrefixTransformNode};

impl HiddenPrefixTransformNode {
    pub fn new(prefix: Vec<ConstantValueNode>) -> Self {
        Self {
            prefix,
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
        let node = HiddenPrefixTransformNode::new(vec![ConstantValueNode::bytes(Base16, "ffff")]);
        assert_eq!(node.prefix, vec![ConstantValueNode::bytes(Base16, "ffff")]);
    }

    #[test]
    fn to_json() {
        let node = HiddenPrefixTransformNode::new(vec![ConstantValueNode::bytes(Base16, "ffff")]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"hiddenPrefixTransformNode","prefix":[{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ffff","encoding":"base16"}}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"hiddenPrefixTransformNode","prefix":[{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ffff","encoding":"base16"}}]}"#;
        let node: HiddenPrefixTransformNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            HiddenPrefixTransformNode::new(vec![ConstantValueNode::bytes(Base16, "ffff")])
        );
    }

    #[test]
    fn to_json_empty() {
        let node = HiddenPrefixTransformNode::new(vec![]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"hiddenPrefixTransformNode"}"#);
    }

    #[test]
    fn from_json_empty() {
        let json = r#"{"kind":"hiddenPrefixTransformNode"}"#;
        let node: HiddenPrefixTransformNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, HiddenPrefixTransformNode::new(vec![]));
    }
}
