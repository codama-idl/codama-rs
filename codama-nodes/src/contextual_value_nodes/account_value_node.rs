use crate::{AccountValueNode, CamelCaseString};

impl AccountValueNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            identifier: identifier.into(),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = AccountValueNode::new("my_account");
        assert_eq!(node.identifier, CamelCaseString::new("myAccount"));
    }

    #[test]
    fn to_json() {
        let node = AccountValueNode::new("myAccount");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"accountValueNode","identifier":"myAccount"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"accountValueNode","identifier":"myAccount"}"#;
        let node: AccountValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, AccountValueNode::new("myAccount"));
    }
}
