use crate::{AccountBumpValueNode, CamelCaseString};

impl AccountBumpValueNode {
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
        let node = AccountBumpValueNode::new("my_account");
        assert_eq!(node.identifier, CamelCaseString::new("myAccount"));
    }

    #[test]
    fn to_json() {
        let node = AccountBumpValueNode::new("myAccount");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"accountBumpValueNode","identifier":"myAccount"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"accountBumpValueNode","identifier":"myAccount"}"#;
        let node: AccountBumpValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, AccountBumpValueNode::new("myAccount"));
    }
}
