use crate::{Integer, IntegerValueNode};

impl IntegerValueNode {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<Integer>,
    {
        Self {
            value: value.into(),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(IntegerValueNode::new(42u8).value, Integer::Unsigned(42));
        assert_eq!(IntegerValueNode::new(-42i32).value, Integer::Signed(-42));
        assert_eq!(
            IntegerValueNode::new(u128::MAX).value,
            Integer::Unsigned(u128::MAX)
        );
    }

    #[test]
    fn to_json() {
        let node = IntegerValueNode::new(-42i64);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"integerValueNode","value":"-42"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"integerValueNode","value":"42"}"#;
        let node: IntegerValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, IntegerValueNode::new(42u32));
    }
}
