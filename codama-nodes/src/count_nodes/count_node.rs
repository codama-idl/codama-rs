use crate::{FixedCountNode, HasKind, NodeUnionTrait, PrefixedCountNode, RemainderCountNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum CountNode {
    Fixed(FixedCountNode),
    Prefixed(PrefixedCountNode),
    Remainder(RemainderCountNode),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind() {
        let node: CountNode = RemainderCountNode::new().into();
        assert_eq!(node.kind(), "remainderCountNode");
    }
}
