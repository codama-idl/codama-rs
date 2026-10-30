use crate::{Decimal, FloatValueNode};

impl FloatValueNode {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<Decimal>,
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
        assert_eq!(FloatValueNode::new(1.5).value.as_str(), "1.5");
        assert_eq!(FloatValueNode::new(-0.0).value.as_str(), "-0");
        assert_eq!(FloatValueNode::new(f64::NAN).value.as_str(), "NaN");
    }

    #[test]
    fn new_from_decimal() {
        let value: Decimal = "3.14159265358979323846".parse().unwrap();
        let node = FloatValueNode::new(value);
        assert_eq!(node.value.as_str(), "3.14159265358979323846");
    }

    #[test]
    fn to_json() {
        let node = FloatValueNode::new(1.5);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"floatValueNode","value":"1.5"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"floatValueNode","value":"1.5"}"#;
        let node: FloatValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, FloatValueNode::new(1.5));
    }

    #[test]
    fn from_json_rejects_a_number() {
        let json = r#"{"kind":"floatValueNode","value":1.5}"#;
        assert!(serde_json::from_str::<FloatValueNode>(json).is_err());
    }
}
