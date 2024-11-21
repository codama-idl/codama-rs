use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct AccountValueNode {
    // Data.
    pub name: CamelCaseString,
}

impl AccountValueNode {
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
        let node = AccountValueNode::new("my_account");
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
    }

    #[test]
    fn to_json() {
        let node = AccountValueNode::new("myAccount");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"accountValueNode","name":"myAccount"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"accountValueNode","name":"myAccount"}"#;
        let node: AccountValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, AccountValueNode::new("myAccount"));
    }
}
