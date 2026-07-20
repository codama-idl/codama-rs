use crate::{PdaSeedValueNode, PdaValueNode, PdaValuePda, PdaValueProgramId};

impl PdaValueNode {
    pub fn new<T>(pda: T, seeds: Vec<PdaSeedValueNode>) -> Self
    where
        T: Into<PdaValuePda>,
    {
        Self {
            pda: Box::new(pda.into()),
            seeds,
            program_id: Box::new(None),
        }
    }

    pub fn new_with_program_id<T, U>(pda: T, seeds: Vec<PdaSeedValueNode>, program_id: U) -> Self
    where
        T: Into<PdaValuePda>,
        U: Into<PdaValueProgramId>,
    {
        Self {
            pda: Box::new(pda.into()),
            seeds,
            program_id: Box::new(Some(program_id.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AccountValueNode, NumberTypeNode, NumberValueNode, PdaLinkNode, PdaNode,
        PublicKeyValueNode, VariablePdaSeedNode, U32,
    };

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
            *node.pda,
            PdaValuePda::PdaLink(PdaLinkNode::new("masterEdition"))
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
            *node.pda,
            PdaValuePda::Pda(PdaNode::new(
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
        // Empty arrays are omitted on write (skip-when-empty convention).
        assert_eq!(
            json,
            r#"{"kind":"pdaValueNode","pda":{"kind":"pdaLinkNode","name":"myPda"}}"#
        );
    }

    #[test]
    fn from_json() {
        // Absent arrays default to empty on read (skip-when-empty convention).
        let json: &str = r#"{"kind":"pdaValueNode","pda":{"kind":"pdaLinkNode","name":"myPda"}}"#;
        let node: PdaValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PdaValueNode::new(PdaLinkNode::new("myPda"), vec![]));
    }

    #[test]
    fn to_json_with_program_id() {
        let node = PdaValueNode::new_with_program_id(
            PdaValuePda::PdaLink(PdaLinkNode::new("myPda")),
            vec![],
            AccountValueNode::new("myProgramAccount"),
        );
        let json = serde_json::to_string(&node).unwrap();
        // Empty arrays are omitted on write (skip-when-empty convention).
        assert_eq!(
            json,
            r#"{"kind":"pdaValueNode","pda":{"kind":"pdaLinkNode","name":"myPda"},"programId":{"kind":"accountValueNode","name":"myProgramAccount"}}"#
        );
    }
}
