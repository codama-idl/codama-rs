use crate::{CamelCaseString, Docs, PdaSeedNode};
use codama_nodes_derive::node;

#[node]
pub struct PdaNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default)]
    #[serde(skip_serializing_if = "Docs::is_empty")]
    pub docs: Docs,
    pub program_id: Option<String>,

    // Children.
    pub seeds: Vec<PdaSeedNode>,
}

impl PdaNode {
    pub fn new<T>(name: T, seeds: Vec<PdaSeedNode>) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            docs: Docs::default(),
            program_id: None,
            seeds: seeds.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ConstantPdaSeedNode, NumberTypeNode, NumberValueNode, PublicKeyTypeNode,
        VariablePdaSeedNode, U8,
    };

    #[test]
    fn new() {
        let node = PdaNode::new(
            "associatedToken",
            vec![
                VariablePdaSeedNode::new("owner", PublicKeyTypeNode::new()).into(),
                ConstantPdaSeedNode::new(NumberTypeNode::le(U8), NumberValueNode::new(42u8)).into(),
                VariablePdaSeedNode::new("mint", PublicKeyTypeNode::new()).into(),
            ],
        );
        assert_eq!(node.name, CamelCaseString::new("associatedToken"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.program_id, None);
        assert_eq!(
            node.seeds,
            vec![
                PdaSeedNode::Variable(VariablePdaSeedNode::new("owner", PublicKeyTypeNode::new())),
                PdaSeedNode::Constant(ConstantPdaSeedNode::new(
                    NumberTypeNode::le(U8),
                    NumberValueNode::new(42u8)
                )),
                PdaSeedNode::Variable(VariablePdaSeedNode::new("mint", PublicKeyTypeNode::new())),
            ]
        );
    }

    #[test]
    fn direct_instantiation() {
        let node = PdaNode {
            name: "myPda".into(),
            docs: Docs::default(),
            program_id: Some("1234..5678".into()),
            seeds: vec![],
        };
        assert_eq!(node.name, CamelCaseString::new("myPda"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.program_id, Some("1234..5678".into()));
        assert_eq!(node.seeds, vec![]);
    }
}
