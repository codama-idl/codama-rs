use codama_nodes::Node;

#[derive(Debug, PartialEq)]
pub struct UnsupportedItemKorok<'a> {
    pub ast: &'a syn::Item,
    pub node: Option<Node>,
}
