use crate::{AccountLinkNode, CamelCaseString, ProgramLinkNode};

impl AccountLinkNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            identifier: identifier.into(),
            program: None,
            plugins: vec![],
        }
    }

    pub fn new_from_program<T>(identifier: T, program: ProgramLinkNode) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            identifier: identifier.into(),
            program: Some(program),
            plugins: vec![],
        }
    }
}

impl From<String> for AccountLinkNode {
    fn from(identifier: String) -> Self {
        Self::new(identifier)
    }
}

impl From<&str> for AccountLinkNode {
    fn from(identifier: &str) -> Self {
        Self::new(identifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = AccountLinkNode::new("my_account");
        assert_eq!(node.identifier, CamelCaseString::new("myAccount"));
    }

    #[test]
    fn new_from_program() {
        let node =
            AccountLinkNode::new_from_program("my_account", ProgramLinkNode::new("my_program"));
        assert_eq!(node.identifier, CamelCaseString::new("myAccount"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
    }

    #[test]
    fn from_string() {
        let node: AccountLinkNode = String::from("my_account").into();
        assert_eq!(node.identifier, CamelCaseString::new("myAccount"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn from_str() {
        let node: AccountLinkNode = "my_account".into();
        assert_eq!(node.identifier, CamelCaseString::new("myAccount"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn to_json() {
        let node = AccountLinkNode::new("myAccount");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"accountLinkNode","identifier":"myAccount"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"accountLinkNode","identifier":"myAccount"}"#;
        let node: AccountLinkNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, AccountLinkNode::new("myAccount"));
    }
}
