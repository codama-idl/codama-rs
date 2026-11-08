use crate::{AccountLinkNode, Identifier, ProgramLinkNode};

impl AccountLinkNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<Identifier>,
    {
        Self {
            identifier: identifier.into(),
            program: None,
            plugins: vec![],
        }
    }

    pub fn new_from_program<T>(identifier: T, program: ProgramLinkNode) -> Self
    where
        T: Into<Identifier>,
    {
        Self {
            identifier: identifier.into(),
            program: Some(program),
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
        let node = AccountLinkNode::new("myAccount");
        assert_eq!(node.identifier, Identifier::from("myAccount"));
    }

    #[test]
    fn new_from_program() {
        let node =
            AccountLinkNode::new_from_program("myAccount", ProgramLinkNode::new("myProgram"));
        assert_eq!(node.identifier, Identifier::from("myAccount"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
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
