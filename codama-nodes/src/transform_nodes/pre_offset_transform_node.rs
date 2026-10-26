use crate::{PreOffsetStrategy, PreOffsetTransformNode};

impl PreOffsetTransformNode {
    pub fn new(offset: i64, strategy: PreOffsetStrategy) -> Self {
        Self {
            offset,
            strategy,
            plugins: vec![],
        }
    }

    pub fn absolute(offset: i64) -> Self {
        Self::new(offset, PreOffsetStrategy::Absolute)
    }

    pub fn padded(offset: i64) -> Self {
        Self::new(offset, PreOffsetStrategy::Padded)
    }

    pub fn relative(offset: i64) -> Self {
        Self::new(offset, PreOffsetStrategy::Relative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = PreOffsetTransformNode::new(-42, PreOffsetStrategy::Absolute);
        assert_eq!(node.offset, -42);
        assert_eq!(node.strategy, PreOffsetStrategy::Absolute);
    }

    #[test]
    fn absolute() {
        let node = PreOffsetTransformNode::absolute(0);
        assert_eq!(node.offset, 0);
        assert_eq!(node.strategy, PreOffsetStrategy::Absolute);
    }

    #[test]
    fn padded() {
        let node = PreOffsetTransformNode::padded(8);
        assert_eq!(node.offset, 8);
        assert_eq!(node.strategy, PreOffsetStrategy::Padded);
    }

    #[test]
    fn relative() {
        let node = PreOffsetTransformNode::relative(-4);
        assert_eq!(node.offset, -4);
        assert_eq!(node.strategy, PreOffsetStrategy::Relative);
    }

    #[test]
    fn to_json() {
        let node = PreOffsetTransformNode::padded(4);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"preOffsetTransformNode","offset":4,"strategy":"padded"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"preOffsetTransformNode","offset":4,"strategy":"padded"}"#;
        let node: PreOffsetTransformNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PreOffsetTransformNode::padded(4));
    }

    #[test]
    fn from_json_without_strategy() {
        let json = r#"{"kind":"preOffsetTransformNode","offset":4}"#;
        let error = serde_json::from_str::<PreOffsetTransformNode>(json).unwrap_err();
        assert!(error.to_string().contains("missing field `strategy`"));
    }
}
