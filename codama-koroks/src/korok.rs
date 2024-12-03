use codama_nodes::Node;

pub trait Korok {
    fn node(&self) -> &Option<Node>;
    fn set_node(&mut self, node: Option<Node>);
}
