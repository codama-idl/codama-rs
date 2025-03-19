use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct PublicKeyValueNode {
    // Data.
    pub public_key: String,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub identifier: Option<CamelCaseString>,
}

impl From<PublicKeyValueNode> for crate::Node {
    fn from(val: PublicKeyValueNode) -> Self {
        crate::Node::Value(val.into())
    }
}

impl PublicKeyValueNode {
    pub fn new<T>(public_key: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            public_key: public_key.into(),
            identifier: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = PublicKeyValueNode::new("6QTRDBBuSgBomH6h5VoKqrw6XZ1ESd7x2dj7ixHc3LWm");
        assert_eq!(
            node.public_key,
            "6QTRDBBuSgBomH6h5VoKqrw6XZ1ESd7x2dj7ixHc3LWm".to_string()
        );
        assert_eq!(node.identifier, None);
    }

    #[test]
    fn direct_instantiation() {
        let node = PublicKeyValueNode {
            public_key: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".into(),
            identifier: Some("spl_token".into()),
        };
        assert_eq!(
            node.public_key,
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string()
        );
        assert_eq!(node.identifier, Some(CamelCaseString::new("splToken")));
    }

    #[test]
    fn to_json() {
        let node = PublicKeyValueNode::new("6QTRDBBuSgBomH6h5VoKqrw6XZ1ESd7x2dj7ixHc3LWm");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"publicKeyValueNode","publicKey":"6QTRDBBuSgBomH6h5VoKqrw6XZ1ESd7x2dj7ixHc3LWm"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"publicKeyValueNode","publicKey":"6QTRDBBuSgBomH6h5VoKqrw6XZ1ESd7x2dj7ixHc3LWm"}"#;
        let node: PublicKeyValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            PublicKeyValueNode::new("6QTRDBBuSgBomH6h5VoKqrw6XZ1ESd7x2dj7ixHc3LWm")
        );
    }
}
