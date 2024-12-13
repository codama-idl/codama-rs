use codama_nodes::Node;

pub trait ApplyToNode {
    fn apply(&self, node: Option<Node>) -> Option<Node> {
        node
    }
}
