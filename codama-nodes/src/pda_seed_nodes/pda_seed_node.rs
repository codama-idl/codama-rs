#[cfg(test)]
mod tests {
    use crate::{ConstantPdaSeedNode, HasKind, NumberTypeNode, NumberValueNode, PdaSeedNode, U8};

    #[test]
    fn kind() {
        let node: PdaSeedNode =
            ConstantPdaSeedNode::new(NumberTypeNode::le(U8), NumberValueNode::new(42u8)).into();
        assert_eq!(node.kind(), "constantPdaSeedNode");
    }
}
