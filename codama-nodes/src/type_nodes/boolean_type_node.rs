use crate::{BooleanTypeNode, IntegerTypeNode, U8};

impl BooleanTypeNode {
    pub fn new<T>(size: T) -> Self
    where
        T: Into<IntegerTypeNode>,
    {
        Self {
            size: size.into(),
            transforms: vec![],
            plugins: vec![],
        }
    }
}

impl Default for BooleanTypeNode {
    fn default() -> Self {
        Self::new(IntegerTypeNode::le(U8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HasTransforms, PostOffsetTransformNode, PreOffsetTransformNode, U64};

    #[test]
    fn new() {
        let node = BooleanTypeNode::new(IntegerTypeNode::le(U64));
        assert_eq!(node.size, IntegerTypeNode::le(U64));
    }

    #[test]
    fn new_with_transformed_size() {
        let size = IntegerTypeNode::le(U64)
            .add_transform(PreOffsetTransformNode::absolute(0))
            .add_transform(PostOffsetTransformNode::pre_offset(0));
        let node = BooleanTypeNode::new(size.clone());
        assert_eq!(node.size, size);
    }

    #[test]
    fn default() {
        let node = BooleanTypeNode::default();
        assert_eq!(node.size, IntegerTypeNode::le(U8));
    }

    #[test]
    fn to_json() {
        let node = BooleanTypeNode::new(IntegerTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"booleanTypeNode","size":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"booleanTypeNode","size":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#;
        let node: BooleanTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, BooleanTypeNode::new(IntegerTypeNode::le(U64)));
    }
}
