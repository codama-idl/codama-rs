use crate::{
    DeriveAttribute, NodeAttribute, NumberAttribute, StringAttribute, UnsupportedAttribute,
};
use codama_errors::{CodamaError, CodamaResult};
use codama_syn_helpers::syn_traits::Path;

#[derive(Debug, PartialEq)]
pub enum Attribute<'a> {
    // E.g. #[derive(Debug, CodamaType)]
    Derive(DeriveAttribute<'a>),
    // E.g. #[node(number_type(u8, le))]
    Node(NodeAttribute<'a>),
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

    fn try_from(attr: &'a syn::Attribute) -> CodamaResult<Self> {
        let path = attr.path();
        match (path.prefix().as_str(), path.last_str().as_str()) {
            ("", "derive") => DeriveAttribute::parse(attr).map(Self::Derive),
            ("" | "codama_macros", "node") => NodeAttribute::parse(attr).map(Self::Node),
            ("" | "codama_macros", "string") => {
                StringAttribute::parse(attr).map(Self::StringModifier)
            }
            ("" | "codama_macros", "number") => {
                NumberAttribute::parse(attr).map(Self::NumberModifier)
            }
            _ => Ok(Self::Unsupported(UnsupportedAttribute::new(attr))),
        }
    }
}

// Do we need this? Or should we be more strict with path recognition?
fn _fallback_to_unsupported<'a>(
    attr: &'a syn::Attribute,
    result: CodamaResult<Attribute<'a>>,
) -> CodamaResult<Attribute<'a>> {
    match result {
        Ok(attr) => Ok(attr),
        Err(_) => Ok(Attribute::Unsupported(UnsupportedAttribute::new(attr))),
    }
}
