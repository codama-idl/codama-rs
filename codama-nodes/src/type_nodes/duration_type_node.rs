use crate::{DurationTypeNode, IntegerTypeNode};

impl DurationTypeNode {
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
    use crate::{Endianness, U32, U64};

    #[test]
    fn new() {
        let node = DurationTypeNode::new(IntegerTypeNode::new(U64, Endianness::Be));
        assert_eq!(node.ticks_per_second, None);
        assert_eq!(node.number, IntegerTypeNode::new(U64, Endianness::Be));
    }

    #[test]
    fn to_json() {
        let node = DurationTypeNode::new(IntegerTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"durationTypeNode","number":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"durationTypeNode","number":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#;
        let node: DurationTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, DurationTypeNode::new(IntegerTypeNode::le(U64)));
    }

    #[test]
    fn to_json_with_ticks_per_second() {
        let node = DurationTypeNode {
            ticks_per_second: Some(1000),
            ..DurationTypeNode::new(IntegerTypeNode::le(U32))
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"durationTypeNode","ticksPerSecond":1000,"number":{"kind":"integerTypeNode","format":"u32","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json_with_ticks_per_second() {
        let json = r#"{"kind":"durationTypeNode","ticksPerSecond":1000,"number":{"kind":"integerTypeNode","format":"u32","endian":"le"}}"#;
        let node: DurationTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node.ticks_per_second, Some(1000));
        assert_eq!(node.number, IntegerTypeNode::le(U32));
    }
}
