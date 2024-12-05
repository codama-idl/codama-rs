use codama_nodes::TypeNode;

#[derive(Debug, PartialEq)]
pub struct TypeAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub node: TypeNode,
}
