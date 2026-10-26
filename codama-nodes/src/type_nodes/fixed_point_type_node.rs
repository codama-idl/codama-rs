use crate::{FixedPointTypeNode, IntegerTypeNode};

impl FixedPointTypeNode {
    pub fn new<T>(number: T, scale: u32) -> Self
    where
        T: Into<IntegerTypeNode>,
    {
        Self {
            scale,
            base: None,
            unit: None,
            number: number.into(),
            display: None,
            transforms: vec![],
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base, StringValueNode, UnitNumberDisplayNode, U128, U64};

    #[test]
    fn new() {
        let node = FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6);
        assert_eq!(node.number, IntegerTypeNode::le(U64));
        assert_eq!(node.scale, 6);
        assert_eq!(node.base, None);
        assert_eq!(node.unit, None);
        assert_eq!(node.display, None);
    }

    #[test]
    fn to_json() {
        let node = FixedPointTypeNode {
            unit: Some("USDC".to_string()),
            ..FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6)
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"fixedPointTypeNode","scale":6,"unit":"USDC","number":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"fixedPointTypeNode","scale":6,"unit":"USDC","number":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#;
        let node: FixedPointTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            FixedPointTypeNode {
                unit: Some("USDC".to_string()),
                ..FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6)
            }
        );
    }

    #[test]
    fn to_json_with_base() {
        let node = FixedPointTypeNode {
            base: Some(Base::Base2),
            ..FixedPointTypeNode::new(IntegerTypeNode::le(U128), 64)
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"fixedPointTypeNode","scale":64,"base":2,"number":{"kind":"integerTypeNode","format":"u128","endian":"le"}}"#
        );
    }

    #[test]
    fn to_json_with_display() {
        let node = FixedPointTypeNode {
            display: Some(UnitNumberDisplayNode::new(StringValueNode::new("SOL"))),
            ..FixedPointTypeNode::new(IntegerTypeNode::le(U64), 9)
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"fixedPointTypeNode","scale":9,"number":{"kind":"integerTypeNode","format":"u64","endian":"le"},"display":{"kind":"unitNumberDisplayNode","unit":{"kind":"stringValueNode","string":"SOL"}}}"#
        );
    }

    #[test]
    fn from_json_without_scale() {
        let json =
            r#"{"kind":"fixedPointTypeNode","number":{"kind":"integerTypeNode","format":"u64"}}"#;
        let error = serde_json::from_str::<FixedPointTypeNode>(json).unwrap_err();
        assert!(error.to_string().contains("missing field `scale`"));
    }
}
