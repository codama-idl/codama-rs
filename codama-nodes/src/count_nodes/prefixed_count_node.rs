use crate::{IntegerTypeNode, PrefixedCountNode};

impl PrefixedCountNode {
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
    use crate::{FixedSizeTransformNode, HasTransforms, U32};

    #[test]
    fn new() {
        let node = PrefixedCountNode::new(IntegerTypeNode::le(U32));
        assert_eq!(node.prefix, IntegerTypeNode::le(U32));
    }

    #[test]
    fn to_json() {
        let node = PrefixedCountNode::new(IntegerTypeNode::le(U32));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"prefixedCountNode","prefix":{"kind":"integerTypeNode","format":"u32","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"prefixedCountNode","prefix":{"kind":"integerTypeNode","format":"u32","endian":"le"}}"#;
        let node: PrefixedCountNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PrefixedCountNode::new(IntegerTypeNode::le(U32)));
    }

    #[test]
    fn new_with_transformed_prefix() {
        let node = PrefixedCountNode::new(
            IntegerTypeNode::le(U32).add_transform(FixedSizeTransformNode::new(8)),
        );
        assert_eq!(
            node.prefix,
            IntegerTypeNode {
                transforms: vec![FixedSizeTransformNode::new(8).into()],
                ..IntegerTypeNode::le(U32)
            }
        );
    }

    #[test]
    fn to_json_with_transformed_prefix() {
        let node = PrefixedCountNode::new(
            IntegerTypeNode::le(U32).add_transform(FixedSizeTransformNode::new(8)),
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"prefixedCountNode","prefix":{"kind":"integerTypeNode","format":"u32","endian":"le","transforms":[{"kind":"fixedSizeTransformNode","size":8}]}}"#
        );
    }

    #[test]
    fn from_json_with_transformed_prefix() {
        let json = r#"{"kind":"prefixedCountNode","prefix":{"kind":"integerTypeNode","format":"u32","endian":"le","transforms":[{"kind":"fixedSizeTransformNode","size":8}]}}"#;
        let node: PrefixedCountNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            PrefixedCountNode::new(
                IntegerTypeNode::le(U32).add_transform(FixedSizeTransformNode::new(8))
            )
        );
    }
}
