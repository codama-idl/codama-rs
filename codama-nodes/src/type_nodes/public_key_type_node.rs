use crate::PublicKeyTypeNode;

impl PublicKeyTypeNode {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AccountLinkNode;

    #[test]
    fn new() {
        let node = PublicKeyTypeNode::new();
        assert_eq!(
            node,
            PublicKeyTypeNode {
                account_link: None,
                transforms: vec![],
                plugins: vec![],
            }
        );
    }

    #[test]
    fn to_json() {
        let node = PublicKeyTypeNode::new();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"publicKeyTypeNode"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"publicKeyTypeNode"}"#;
        let node: PublicKeyTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PublicKeyTypeNode::new());
    }

    #[test]
    fn to_json_with_account_link() {
        let node = PublicKeyTypeNode {
            account_link: Some(AccountLinkNode::new("mint")),
            ..PublicKeyTypeNode::new()
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"publicKeyTypeNode","accountLink":{"kind":"accountLinkNode","identifier":"mint"}}"#
        );
    }
}
