use crate::{CamelCaseString, Docs, PdaNode, PdaSeedNode};

impl PdaNode {
    pub fn new<T>(identifier: T, seeds: Vec<PdaSeedNode>) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            identifier: identifier.into(),
            docs: Docs::default(),
            program_id: None,
            seeds,
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ConstantPdaSeedNode, IntegerTypeNode, IntegerValueNode, PublicKeyTypeNode,
        VariablePdaSeedNode, U8,
    };

    #[test]
    fn new() {
        let node = PdaNode::new(
            "associatedToken",
            vec![
                VariablePdaSeedNode::new("owner", PublicKeyTypeNode::new()).into(),
                ConstantPdaSeedNode::new(IntegerTypeNode::le(U8), IntegerValueNode::new(42u8))
                    .into(),
                VariablePdaSeedNode::new("mint", PublicKeyTypeNode::new()).into(),
            ],
        );
        assert_eq!(node.identifier, CamelCaseString::new("associatedToken"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.program_id, None);
        assert_eq!(
            node.seeds,
            vec![
                PdaSeedNode::Variable(VariablePdaSeedNode::new("owner", PublicKeyTypeNode::new())),
                PdaSeedNode::Constant(ConstantPdaSeedNode::new(
                    IntegerTypeNode::le(U8),
                    IntegerValueNode::new(42u8)
                )),
                PdaSeedNode::Variable(VariablePdaSeedNode::new("mint", PublicKeyTypeNode::new())),
            ]
        );
    }

    #[test]
    fn direct_instantiation() {
        let node = PdaNode {
            identifier: "myPda".into(),
            docs: Docs::default(),
            program_id: Some("1234..5678".into()),
            seeds: vec![],
            plugins: vec![],
        };
        assert_eq!(node.identifier, CamelCaseString::new("myPda"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.program_id, Some("1234..5678".into()));
        assert_eq!(node.seeds, vec![]);
    }

    #[test]
    fn to_json() {
        let node = PdaNode::new("myPda", vec![]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"pdaNode","identifier":"myPda"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"pdaNode","identifier":"myPda"}"#;
        let node: PdaNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PdaNode::new("myPda", vec![]));
    }
}
