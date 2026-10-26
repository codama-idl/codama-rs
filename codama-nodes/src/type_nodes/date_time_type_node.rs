use crate::{DateTimeTypeNode, IntegerTypeNode};

impl DateTimeTypeNode {
    pub fn new<T>(number: T) -> Self
    where
        T: Into<IntegerTypeNode>,
    {
        Self {
            ticks_per_second: None,
            number: number.into(),
            transforms: vec![],
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Endianness, HasTransforms, PostOffsetTransformNode, PreOffsetTransformNode, TransformNode,
        I64, U64,
    };

    #[test]
    fn new() {
        let node = DateTimeTypeNode::new(IntegerTypeNode::new(U64, Endianness::Be));
        assert_eq!(node.ticks_per_second, None);
        assert_eq!(node.number, IntegerTypeNode::new(U64, Endianness::Be));
    }

    #[test]
    fn new_with_transforms() {
        let node = DateTimeTypeNode::new(IntegerTypeNode::le(U64))
            .add_transform(PreOffsetTransformNode::absolute(0))
            .add_transform(PostOffsetTransformNode::pre_offset(0));
        assert_eq!(node.number, IntegerTypeNode::le(U64));
        assert_eq!(
            node.transforms,
            vec![
                TransformNode::PreOffset(PreOffsetTransformNode::absolute(0)),
                TransformNode::PostOffset(PostOffsetTransformNode::pre_offset(0)),
            ]
        );
    }

    #[test]
    fn to_json() {
        let node = DateTimeTypeNode::new(IntegerTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"dateTimeTypeNode","number":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"dateTimeTypeNode","number":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#;
        let node: DateTimeTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, DateTimeTypeNode::new(IntegerTypeNode::le(U64)));
    }

    #[test]
    fn to_json_with_ticks_per_second() {
        let node = DateTimeTypeNode {
            ticks_per_second: Some(1000),
            ..DateTimeTypeNode::new(IntegerTypeNode::le(I64))
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"dateTimeTypeNode","ticksPerSecond":1000,"number":{"kind":"integerTypeNode","format":"i64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json_with_ticks_per_second() {
        let json = r#"{"kind":"dateTimeTypeNode","ticksPerSecond":1000,"number":{"kind":"integerTypeNode","format":"i64","endian":"le"}}"#;
        let node: DateTimeTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node.ticks_per_second, Some(1000));
        assert_eq!(node.number, IntegerTypeNode::le(I64));
    }
}
