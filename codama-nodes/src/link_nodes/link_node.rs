#[cfg(test)]
mod tests {
    use crate::test_utils::id;
    use crate::{HasKind, LinkNode, ProgramLinkNode};

    #[test]
    fn kind() {
        let node: LinkNode = ProgramLinkNode::new(id("myProgram")).into();
        assert_eq!(node.kind(), "programLinkNode");
    }
}
