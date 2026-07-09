use crate::{AccountFieldValueNode, CamelCaseString};

impl AccountFieldValueNode {
    pub fn new<T>(account: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            account: account.into(),
            path: None,
        }
    }

    pub fn with_path<T, U>(account: T, path: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<CamelCaseString>,
    {
        Self {
            account: account.into(),
            path: Some(path.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = AccountFieldValueNode::new("my_account");
        assert_eq!(node.account, CamelCaseString::new("myAccount"));
        assert_eq!(node.path, None);
    }

    #[test]
    fn with_path() {
        let node = AccountFieldValueNode::with_path("my_account", "my_field");
        assert_eq!(node.account, CamelCaseString::new("myAccount"));
        assert_eq!(node.path, Some(CamelCaseString::new("myField")));
    }

    #[test]
    fn to_json() {
        let node = AccountFieldValueNode::new("myAccount");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"accountFieldValueNode","account":"myAccount"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"accountFieldValueNode","account":"myAccount"}"#;
        let node: AccountFieldValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, AccountFieldValueNode::new("myAccount"));
    }

    #[test]
    fn to_json_with_path() {
        let node = AccountFieldValueNode::with_path("myAccount", "myField");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"accountFieldValueNode","account":"myAccount","path":"myField"}"#
        );
    }

    #[test]
    fn from_json_with_path() {
        let json = r#"{"kind":"accountFieldValueNode","account":"myAccount","path":"myField"}"#;
        let node: AccountFieldValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            AccountFieldValueNode::with_path("myAccount", "myField")
        );
    }
}
