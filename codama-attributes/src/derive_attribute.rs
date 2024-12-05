#[derive(Debug, PartialEq)]
pub struct DeriveAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub derives: Vec<&'a syn::Path>,
}
