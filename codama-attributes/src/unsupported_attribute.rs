#[derive(Debug, PartialEq)]
pub struct UnsupportedAttribute<'a> {
    pub ast: &'a syn::Attribute,
}
