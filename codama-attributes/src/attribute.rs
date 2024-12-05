use crate::{
    DeriveAttribute, NumberAttribute, StringAttribute, TypeAttribute, UnsupportedAttribute,
};
use codama_errors::{CodamaError, CodamaResult};

#[derive(Debug, PartialEq)]
pub enum Attribute<'a> {
    // E.g. #[derive(Debug, CodamaType)]
    Derive(DeriveAttribute<'a>),
    // E.g. #[type(numberTypeNode(u8, le))]
    Type(TypeAttribute<'a>),
    // E.g. #[string(base64)]
    StringModifier(StringAttribute<'a>),
    // E.g. #[number(be)]
    NumberModifier(NumberAttribute<'a>),
    // E.g. #[some_unsupported_attribute = 42]
    Unsupported(UnsupportedAttribute<'a>),
}

impl<'a> Attribute<'a> {
    pub fn parse<T: TryInto<Self, Error = CodamaError>>(attr: T) -> CodamaResult<Self> {
        attr.try_into()
    }
}

impl<'a> TryFrom<&'a syn::Attribute> for Attribute<'a> {
    type Error = CodamaError;

    fn try_from(_attr: &'a syn::Attribute) -> CodamaResult<Self> {
        unimplemented!()
    }
}
