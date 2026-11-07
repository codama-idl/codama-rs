use crate::{Identifier, InjectedValueNode, ValueNode};

impl InjectedValueNode {
    pub fn new<T>(key: T) -> Self
    where
        T: Into<Identifier>,
    {
        Self {
            key: key.into(),
            fallback: Box::new(None),
            plugins: vec![],
        }
    }

    pub fn with_fallback<T, U>(key: T, fallback: U) -> Self
    where
        T: Into<Identifier>,
        U: Into<ValueNode>,
    {
        Self {
            key: key.into(),
            fallback: Box::new(Some(fallback.into())),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::id;
    use crate::IntegerValueNode;

    #[test]
    fn new() {
        let node = InjectedValueNode::new(id("myKey"));
        assert_eq!(node.key, id("myKey"));
        assert_eq!(*node.fallback, None);
    }

    #[test]
    fn with_fallback() {
        let node = InjectedValueNode::with_fallback(id("myKey"), IntegerValueNode::new(42));
        assert_eq!(node.key, id("myKey"));
        assert_eq!(
            *node.fallback,
            Some(ValueNode::Integer(IntegerValueNode::new(42)))
        );
    }

    #[test]
    fn to_json() {
        let node = InjectedValueNode::new(id("myKey"));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"injectedValueNode","key":"myKey"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"injectedValueNode","key":"myKey"}"#;
        let node: InjectedValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, InjectedValueNode::new(id("myKey")));
    }

    #[test]
    fn to_json_with_fallback() {
        let node = InjectedValueNode::with_fallback(id("myKey"), IntegerValueNode::new(42));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"injectedValueNode","key":"myKey","fallback":{"kind":"integerValueNode","value":"42"}}"#
        );
    }

    #[test]
    fn from_json_with_fallback() {
        let json = r#"{"kind":"injectedValueNode","key":"myKey","fallback":{"kind":"integerValueNode","value":"42"}}"#;
        let node: InjectedValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InjectedValueNode::with_fallback(id("myKey"), IntegerValueNode::new(42u32))
        );
    }
}
