use crate::{IntegerTypeNode, SizePrefixTransformNode};

impl SizePrefixTransformNode {
    pub fn new<T>(prefix: T) -> Self
    where
        T: Into<IntegerTypeNode>,
    {
        Self {
            prefix: prefix.into(),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::U32;

    #[test]
    fn new() {
        let node = SizePrefixTransformNode::new(IntegerTypeNode::le(U32));
        assert_eq!(node.prefix, IntegerTypeNode::le(U32));
    }

    #[test]
    fn to_json() {
        let node = SizePrefixTransformNode::new(IntegerTypeNode::le(U32));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"sizePrefixTransformNode","prefix":{"kind":"integerTypeNode","format":"u32","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"sizePrefixTransformNode","prefix":{"kind":"integerTypeNode","format":"u32","endian":"le"}}"#;
        let node: SizePrefixTransformNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, SizePrefixTransformNode::new(IntegerTypeNode::le(U32)));
    }
}
