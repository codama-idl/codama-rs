use crate::{PostOffsetStrategy, PostOffsetTransformNode};

impl PostOffsetTransformNode {
    pub fn new(offset: i64, strategy: PostOffsetStrategy) -> Self {
        Self {
            offset,
            strategy,
            plugins: vec![],
        }
    }

    pub fn absolute(offset: i64) -> Self {
        Self::new(offset, PostOffsetStrategy::Absolute)
    }

    pub fn padded(offset: i64) -> Self {
        Self::new(offset, PostOffsetStrategy::Padded)
    }

    pub fn pre_offset(offset: i64) -> Self {
        Self::new(offset, PostOffsetStrategy::PreOffset)
    }

    pub fn relative(offset: i64) -> Self {
        Self::new(offset, PostOffsetStrategy::Relative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = PostOffsetTransformNode::new(-42, PostOffsetStrategy::Absolute);
        assert_eq!(node.offset, -42);
        assert_eq!(node.strategy, PostOffsetStrategy::Absolute);
    }

    #[test]
    fn absolute() {
        let node = PostOffsetTransformNode::absolute(0);
        assert_eq!(node.offset, 0);
        assert_eq!(node.strategy, PostOffsetStrategy::Absolute);
    }

    #[test]
    fn padded() {
        let node = PostOffsetTransformNode::padded(8);
        assert_eq!(node.offset, 8);
        assert_eq!(node.strategy, PostOffsetStrategy::Padded);
    }

    #[test]
    fn pre_offset() {
        let node = PostOffsetTransformNode::pre_offset(2);
        assert_eq!(node.offset, 2);
        assert_eq!(node.strategy, PostOffsetStrategy::PreOffset);
    }

    #[test]
    fn relative() {
        let node = PostOffsetTransformNode::relative(-4);
        assert_eq!(node.offset, -4);
        assert_eq!(node.strategy, PostOffsetStrategy::Relative);
    }

    #[test]
    fn to_json() {
        let node = PostOffsetTransformNode::pre_offset(-2);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"postOffsetTransformNode","offset":-2,"strategy":"preOffset"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"postOffsetTransformNode","offset":-2,"strategy":"preOffset"}"#;
        let node: PostOffsetTransformNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PostOffsetTransformNode::pre_offset(-2));
    }

    #[test]
    fn from_json_without_strategy() {
        let json = r#"{"kind":"postOffsetTransformNode","offset":4}"#;
        let error = serde_json::from_str::<PostOffsetTransformNode>(json).unwrap_err();
        assert!(error.to_string().contains("missing field `strategy`"));
    }
}
