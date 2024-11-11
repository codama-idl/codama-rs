use crate::{
    internals::{ParsingError, ParsingResult},
    nodes::TypeNode,
};

pub enum Attribute<'a> {
    Type(TypeAttribute<'a>),
    StringModifier(StringModifierAttribute<'a>),
    NumberModifier(NumberModifierAttribute<'a>),
    Unsupported(UnsupportedAttribute<'a>),
}

impl<'a> Attribute<'a> {
    pub fn parse_all(attrs: &'a Vec<syn::Attribute>) -> ParsingResult<Vec<Self>> {
        attrs.iter().map(Self::try_from).collect()
    }
}

impl<'a> TryFrom<&'a syn::Attribute> for Attribute<'a> {
    type Error = ParsingError;

    fn try_from(ast: &'a syn::Attribute) -> ParsingResult<Self> {
        // TODO: implement.
        Ok(Attribute::Unsupported(UnsupportedAttribute { ast }))
    }
}

pub struct TypeAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub node: TypeNode,
}

pub struct StringModifierAttribute<'a> {
    pub ast: &'a syn::Attribute,
}

pub struct NumberModifierAttribute<'a> {
    pub ast: &'a syn::Attribute,
}

pub struct UnsupportedAttribute<'a> {
    pub ast: &'a syn::Attribute,
}
