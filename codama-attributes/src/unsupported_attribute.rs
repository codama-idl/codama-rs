#[derive(Debug, PartialEq)]
pub struct UnsupportedAttribute<'a> {
    pub ast: &'a syn::Attribute,
}

impl<'a> UnsupportedAttribute<'a> {
    pub fn new(ast: &'a syn::Attribute) -> Self {
        Self { ast }
    }
}
