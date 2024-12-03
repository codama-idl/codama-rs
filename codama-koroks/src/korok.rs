use codama_nodes::Node;
use std::fmt::Debug;

pub trait Korok: Debug {
    fn node(&self) -> &Option<Node>;
    fn set_node(&mut self, node: Option<Node>);
}
