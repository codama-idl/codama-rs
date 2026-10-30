#[cfg(test)]
mod tests {
    use crate::{FixedSizeTransformNode, HasKind, PostOffsetTransformNode, TransformNode};

    #[test]
    fn kind() {
        let node: TransformNode = FixedSizeTransformNode::new(42).into();
        assert_eq!(node.kind(), "fixedSizeTransformNode");
    }

    #[test]
    fn from_json() {
        let json = r#"[{"kind":"fixedSizeTransformNode","size":42},{"kind":"postOffsetTransformNode","offset":2,"strategy":"relative"}]"#;
        let nodes: Vec<TransformNode> = serde_json::from_str(json).unwrap();
        assert_eq!(
            nodes,
            vec![
                TransformNode::FixedSize(FixedSizeTransformNode::new(42)),
                TransformNode::PostOffset(PostOffsetTransformNode::relative(2)),
            ]
        );
        assert_eq!(serde_json::to_string(&nodes).unwrap(), json);
    }
}
