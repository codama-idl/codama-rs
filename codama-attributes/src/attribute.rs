use crate::{CodamaAttribute, DeriveAttribute, UnsupportedAttribute};
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub enum Attribute<'a> {
    // E.g. #[derive(Debug, CodamaType)]
    Derive(DeriveAttribute<'a>),
    // E.g. #[codama(node(number_type(u8, le)))]
    Codama(CodamaAttribute<'a>),
    // E.g. #[some_unsupported_attribute = 42]
    Unsupported(UnsupportedAttribute<'a>),
}

impl<'a> Attribute<'a> {
    pub fn parse<T: TryInto<Self, Error = syn::Error>>(attr: T) -> syn::Result<Self> {
        attr.try_into()
    }
}

impl<'a> TryFrom<&'a syn::Attribute> for Attribute<'a> {
    type Error = syn::Error;

    fn try_from(attr: &'a syn::Attribute) -> syn::Result<Self> {
        let path = attr.path();
        match (path.prefix().as_str(), path.last_str().as_str()) {
            ("", "derive") => Ok(Attribute::Derive(attr.try_into()?)),
            ("" | "codama_macros" | "codama", "codama") => Ok(Attribute::Codama(attr.try_into()?)),
            _ => Ok(Self::Unsupported(UnsupportedAttribute::new(attr))),
        }
    }
}
