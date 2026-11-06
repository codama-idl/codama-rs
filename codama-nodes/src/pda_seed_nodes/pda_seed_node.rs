#[cfg(test)]
mod tests {
    use crate::{ConstantPdaSeedNode, HasKind, IntegerTypeNode, IntegerValueNode, PdaSeedNode, U8};

    #[test]
    fn kind() {
        let node: PdaSeedNode =
            ConstantPdaSeedNode::new(IntegerTypeNode::le(U8), IntegerValueNode::new(42u8)).into();
        assert_eq!(node.kind(), "constantPdaSeedNode");
    }
}
