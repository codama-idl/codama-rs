use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct PublicKeyValueNode {
    // Data.
    pub public_key: String,
    pub identifier: Option<CamelCaseString>,
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
}
