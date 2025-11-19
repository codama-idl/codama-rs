use crate::{AccountValueNode, ArgumentValueNode, PdaLinkNode, PdaNode, PdaSeedValueNode};
use codama_nodes_derive::{node, node_union};

#[node]
pub struct PdaValueNode {
    // Children.
    pub pda: PdaValue,
    pub seeds: Vec<PdaSeedValueNode>,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub program_id: Box<Option<PdaProgramIdValueNode>>,
}

#[node_union]
pub enum PdaProgramIdValueNode {
    Account(AccountValueNode),
    Argument(ArgumentValueNode),
}

impl From<PdaValueNode> for crate::Node {
    fn from(val: PdaValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl PdaValueNode {
    pub fn new<T>(pda: T, seeds: Vec<PdaSeedValueNode>) -> Self
    where
        T: Into<PdaValue>,
    {
        Self {
            pda: pda.into(),
            seeds,
            program_id: Box::new(None),
        }
    }

    pub fn new_with_program_id<T, U>(pda: T, seeds: Vec<PdaSeedValueNode>, program_id: U) -> Self
    where
        T: Into<PdaValue>,
        U: Into<PdaProgramIdValueNode>,
    {
        Self {
            pda: pda.into(),
            seeds,
            program_id: Box::new(Some(program_id.into())),
        }
    }
}

#[node_union]
pub enum PdaValue {
    Linked(PdaLinkNode),
    Nested(PdaNode),
}

#[cfg(test)]
mod tests {
    use crate::{NumberTypeNode, NumberValueNode, PublicKeyValueNode, VariablePdaSeedNode, U32};

    use super::*;

    #[test]
    fn new_linked() {
        let node = PdaValueNode::new(
            PdaLinkNode::new("masterEdition"),
            vec![
                PdaSeedValueNode::new(
                    "mint",
                    PublicKeyValueNode::new("33QJ9VtGKRS7wstQiwuigk1cBVYEPp3XBCC1g9WkDFEE"),
                ),
                PdaSeedValueNode::new("edition", NumberValueNode::new(42)),
            ],
        );
        assert_eq!(
            node.pda,
            PdaValue::Linked(PdaLinkNode::new("masterEdition"))
        );
        assert_eq!(
            node.seeds,
            vec![
                PdaSeedValueNode::new(
                    "mint",
                    PublicKeyValueNode::new("33QJ9VtGKRS7wstQiwuigk1cBVYEPp3XBCC1g9WkDFEE")
                ),
                PdaSeedValueNode::new("edition", NumberValueNode::new(42)),
            ]
        );
    }

    #[test]
    fn new_nested() {
        let node = PdaValueNode::new(
            PdaNode::new(
                "counter",
                vec![VariablePdaSeedNode::new("value", NumberTypeNode::le(U32)).into()],
            ),
            vec![PdaSeedValueNode::new("value", NumberValueNode::new(42))],
        );
        assert_eq!(
            node.pda,
            PdaValue::Nested(PdaNode::new(
                "counter",
                vec![VariablePdaSeedNode::new("value", NumberTypeNode::le(U32)).into()],
            ))
        );
        assert_eq!(
            node.seeds,
            vec![PdaSeedValueNode::new("value", NumberValueNode::new(42)),]
        );
    }

    #[test]
    fn to_json() {
        let node = PdaValueNode::new(PdaLinkNode::new("myPda"), vec![]);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"pdaValueNode","pda":{"kind":"pdaLinkNode","name":"myPda"},"seeds":[]}"#
        );
    }

    #[test]
    fn from_json() {
        let json: &str =
            r#"{"kind":"pdaValueNode","pda":{"kind":"pdaLinkNode","name":"myPda"},"seeds":[]}"#;
        let node: PdaValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PdaValueNode::new(PdaLinkNode::new("myPda"), vec![]));
    }

    #[test]
    fn to_json_with_program_id() {
        let node = PdaValueNode::new_with_program_id(
            PdaValue::Linked(PdaLinkNode::new("myPda")),
            vec![],
            AccountValueNode::new("myProgramAccount"),
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"pdaValueNode","pda":{"kind":"pdaLinkNode","name":"myPda"},"seeds":[],"programId":{"kind":"accountValueNode","name":"myProgramAccount"}}"#
        );
    }
}
