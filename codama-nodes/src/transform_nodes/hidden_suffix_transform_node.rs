use crate::{ConstantValueNode, HiddenSuffixTransformNode};

impl HiddenSuffixTransformNode {
    pub fn new(suffix: Vec<ConstantValueNode>) -> Self {
        Self {
            suffix,
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
        let node = HiddenSuffixTransformNode::new(vec![ConstantValueNode::bytes(Base16, "ffff")]);
        assert_eq!(node.suffix, vec![ConstantValueNode::bytes(Base16, "ffff")]);
    }

    #[test]
    fn to_json() {
        let node = HiddenSuffixTransformNode::new(vec![ConstantValueNode::bytes(Base16, "ffff")]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"hiddenSuffixTransformNode","suffix":[{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ffff","encoding":"base16"}}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"hiddenSuffixTransformNode","suffix":[{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ffff","encoding":"base16"}}]}"#;
        let node: HiddenSuffixTransformNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            HiddenSuffixTransformNode::new(vec![ConstantValueNode::bytes(Base16, "ffff")])
        );
    }

    #[test]
    fn to_json_empty() {
        let node = HiddenSuffixTransformNode::new(vec![]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"hiddenSuffixTransformNode"}"#);
    }

    #[test]
    fn from_json_empty() {
        let json = r#"{"kind":"hiddenSuffixTransformNode"}"#;
        let node: HiddenSuffixTransformNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, HiddenSuffixTransformNode::new(vec![]));
    }
}
