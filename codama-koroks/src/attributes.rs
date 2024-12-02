use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::TypeNode;

#[derive(Debug, PartialEq)]
pub enum Attribute<'a> {
    Type(TypeAttribute<'a>),
    StringModifier(StringModifierAttribute<'a>),
    NumberModifier(NumberModifierAttribute<'a>),
    Unsupported(UnsupportedAttribute<'a>),
}

impl<'a> Attribute<'a> {
    pub fn parse_all(attrs: &'a Vec<syn::Attribute>) -> CodamaResult<Vec<Self>> {
        attrs.iter().map(Self::try_from).collect()
    }
}

impl<'a> TryFrom<&'a syn::Attribute> for Attribute<'a> {
    type Error = CodamaError;

    fn try_from(ast: &'a syn::Attribute) -> CodamaResult<Self> {
        // TODO: implement.
        Ok(Attribute::Unsupported(UnsupportedAttribute { ast }))
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub node: TypeNode,
}

#[derive(Debug, PartialEq)]
pub struct StringModifierAttribute<'a> {
    pub ast: &'a syn::Attribute,
}

#[derive(Debug, PartialEq)]
pub struct NumberModifierAttribute<'a> {
    pub ast: &'a syn::Attribute,
}

#[derive(Debug, PartialEq)]
pub struct UnsupportedAttribute<'a> {
    pub ast: &'a syn::Attribute,
}
