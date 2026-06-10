#[cfg(test)]
mod tests {
    use crate::{ContextualValueNode, HasKind, ProgramIdValueNode, RegisteredContextualValueNode};

    #[test]
    fn kind_from_standalone() {
        let node: ContextualValueNode = ProgramIdValueNode::new().into();
        assert_eq!(node.kind(), "programIdValueNode");
    }

    #[test]
    fn kind_from_registered() {
        let node: RegisteredContextualValueNode = ProgramIdValueNode::new().into();
        assert_eq!(node.kind(), "programIdValueNode");
    }
}
