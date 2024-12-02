use codama_nodes::Node;

#[derive(Debug, PartialEq)]
pub struct TypeKorok<'a> {
    pub ast: &'a syn::Type,
    pub node: Option<Node>,
}

impl<'a> TypeKorok<'a> {
    pub fn new(ast: &'a syn::Type) -> Self {
        Self { ast, node: None }
    }
}
