use crate::{AccountDataValueNode, CamelCaseString, Path};

impl AccountDataValueNode {
    pub fn new<T>(account: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            account: account.into(),
            path: None,
            plugins: vec![],
        }
    }

    pub fn with_path<T, U>(account: T, path: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<Path>,
    {
        Self {
            account: account.into(),
            path: Some(path.into()),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(value: &str) -> Path {
        value.parse().unwrap()
    }

    #[test]
    fn new() {
        let node = AccountDataValueNode::new("my_account");
        assert_eq!(node.account, CamelCaseString::new("myAccount"));
        assert_eq!(node.path, None);
    }

    #[test]
    fn with_path() {
        let node = AccountDataValueNode::with_path("my_account", path("my_field"));
        assert_eq!(node.account, CamelCaseString::new("myAccount"));
        assert_eq!(node.path, Some(path("my_field")));
    }

    #[test]
    fn to_json() {
        let node = AccountDataValueNode::new("mint");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"accountDataValueNode","account":"mint"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"accountDataValueNode","account":"mint"}"#;
        let node: AccountDataValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, AccountDataValueNode::new("mint"));
    }

    #[test]
    fn to_json_with_path() {
        let node = AccountDataValueNode::with_path("vault", path("state.balances[0]"));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"accountDataValueNode","account":"vault","path":"state.balances[0]"}"#
        );
    }

    #[test]
    fn from_json_with_path() {
        let json =
            r#"{"kind":"accountDataValueNode","account":"vault","path":"state.balances[0]"}"#;
        let node: AccountDataValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            AccountDataValueNode::with_path("vault", path("state.balances[0]"))
        );
    }

    #[test]
    fn from_json_rejects_an_invalid_path() {
        let json = r#"{"kind":"accountDataValueNode","account":"vault","path":"state..balances"}"#;
        let error = serde_json::from_str::<AccountDataValueNode>(json).unwrap_err();
        assert!(error
            .to_string()
            .contains("expected a path expression string"));
    }
}
