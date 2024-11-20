use crate::{PdaLinkNode, PdaNode, PdaSeedValueNode};
use codama_nodes_derive::{IntoEnum, Node};

#[derive(Node, Debug, PartialEq, Clone)]
pub struct PdaValueNode {
    // Children.
    pub pda: PdaValue,
    pub seeds: Vec<PdaSeedValueNode>,
}

impl PdaValueNode {
    pub fn new<T>(pda: T, seeds: Vec<PdaSeedValueNode>) -> Self
    where
        T: Into<PdaValue>,
    {
        Self {
            pda: pda.into(),
            seeds,
        }
    }
}

#[derive(IntoEnum, Debug, PartialEq, Clone)]
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
}
