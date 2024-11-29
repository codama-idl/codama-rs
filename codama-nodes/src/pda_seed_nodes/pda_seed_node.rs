use crate::{ConstantPdaSeedNode, NodeTrait, NodeUnionTrait, VariablePdaSeedNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum PdaSeedNode {
    Constant(ConstantPdaSeedNode),
    Variable(VariablePdaSeedNode),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NodeUnionTrait, NumberTypeNode, NumberValueNode, U8};

    #[test]
    fn kind() {
        let node: PdaSeedNode =
            ConstantPdaSeedNode::new(NumberTypeNode::le(U8), NumberValueNode::new(42u8)).into();
        assert_eq!(node.kind(), "constantPdaSeedNode");
    }
}
