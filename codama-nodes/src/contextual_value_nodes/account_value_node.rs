use crate::{AccountValueNode, Identifier};

impl AccountValueNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<Identifier>,
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
    use crate::Identifier;

    #[test]
    fn new() {
        let node = AccountValueNode::new("myAccount");
        assert_eq!(node.identifier, Identifier::from("myAccount"));
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
