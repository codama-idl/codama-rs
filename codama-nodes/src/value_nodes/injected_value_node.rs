use crate::{CamelCaseString, InjectedValueNode, ValueNode};

impl InjectedValueNode {
    pub fn new<T>(key: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            key: key.into(),
            fallback: Box::new(None),
        }
    }

    pub fn with_fallback<T, U>(key: T, fallback: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<ValueNode>,
    {
        Self {
            key: key.into(),
            fallback: Box::new(Some(fallback.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NumberValueNode;

    #[test]
    fn new() {
        let node = InjectedValueNode::new("my_key");
        assert_eq!(node.key, CamelCaseString::new("myKey"));
        assert_eq!(*node.fallback, None);
    }

    #[test]
    fn with_fallback() {
        let node = InjectedValueNode::with_fallback("my_key", NumberValueNode::new(42));
        assert_eq!(node.key, CamelCaseString::new("myKey"));
        assert_eq!(
            *node.fallback,
            Some(ValueNode::Number(NumberValueNode::new(42)))
        );
    }

    #[test]
    fn to_json() {
        let node = InjectedValueNode::new("myKey");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"injectedValueNode","key":"myKey"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"injectedValueNode","key":"myKey"}"#;
        let node: InjectedValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, InjectedValueNode::new("myKey"));
    }

    #[test]
    fn to_json_with_fallback() {
        let node = InjectedValueNode::with_fallback("myKey", NumberValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"injectedValueNode","key":"myKey","fallback":{"kind":"numberValueNode","number":42}}"#
        );
    }

    #[test]
    fn from_json_with_fallback() {
        let json = r#"{"kind":"injectedValueNode","key":"myKey","fallback":{"kind":"numberValueNode","number":42}}"#;
        let node: InjectedValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InjectedValueNode::with_fallback("myKey", NumberValueNode::new(42u32))
        );
    }
}
