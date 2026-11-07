use crate::{Docs, Identifier, PdaNode, PdaSeedNode};

impl PdaNode {
    pub fn new<T>(identifier: T, seeds: Vec<PdaSeedNode>) -> Self
    where
        T: Into<Identifier>,
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
    use crate::test_utils::id;
    use crate::{
        ConstantPdaSeedNode, IntegerTypeNode, IntegerValueNode, PublicKeyTypeNode,
        VariablePdaSeedNode, U8,
    };

    #[test]
    fn new() {
        let node = PdaNode::new(
            id("associatedToken"),
            vec![
                VariablePdaSeedNode::new(id("owner"), PublicKeyTypeNode::new()).into(),
                ConstantPdaSeedNode::new(IntegerTypeNode::le(U8), IntegerValueNode::new(42u8))
                    .into(),
                VariablePdaSeedNode::new(id("mint"), PublicKeyTypeNode::new()).into(),
            ],
        );
        assert_eq!(node.identifier, id("associatedToken"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.program_id, None);
        assert_eq!(
            node.seeds,
            vec![
                PdaSeedNode::Variable(VariablePdaSeedNode::new(
                    id("owner"),
                    PublicKeyTypeNode::new()
                )),
                PdaSeedNode::Constant(ConstantPdaSeedNode::new(
                    IntegerTypeNode::le(U8),
                    IntegerValueNode::new(42u8)
                )),
                PdaSeedNode::Variable(VariablePdaSeedNode::new(
                    id("mint"),
                    PublicKeyTypeNode::new()
                )),
            ]
        );
    }

    #[test]
    fn direct_instantiation() {
        let node = PdaNode {
            identifier: id("myPda"),
            docs: Docs::default(),
            program_id: Some("1234..5678".into()),
            seeds: vec![],
            plugins: vec![],
        };
        assert_eq!(node.identifier, id("myPda"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.program_id, Some("1234..5678".into()));
        assert_eq!(node.seeds, vec![]);
    }

    #[test]
    fn to_json() {
        let node = PdaNode::new(id("myPda"), vec![]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"pdaNode","identifier":"myPda"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"pdaNode","identifier":"myPda"}"#;
        let node: PdaNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PdaNode::new(id("myPda"), vec![]));
    }
}
