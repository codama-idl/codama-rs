use crate::Korok;
use codama_nodes::Node;

#[derive(Debug, PartialEq)]
pub struct UnsupportedItemKorok<'a> {
    pub ast: &'a syn::Item,
    pub node: Option<Node>,
}

impl Korok for UnsupportedItemKorok<'_> {
    fn node(&self) -> &Option<Node> {
        &self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }
}
