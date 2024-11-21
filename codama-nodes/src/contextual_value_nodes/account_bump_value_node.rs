use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct AccountBumpValueNode {
    // Data.
    pub name: CamelCaseString,
}

impl AccountBumpValueNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self { name: name.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = AccountBumpValueNode::new("my_account");
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
    }

    #[test]
    fn to_json() {
        let node = AccountBumpValueNode::new("myAccount");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"accountBumpValueNode","name":"myAccount"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"accountBumpValueNode","name":"myAccount"}"#;
        let node: AccountBumpValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, AccountBumpValueNode::new("myAccount"));
    }
}
