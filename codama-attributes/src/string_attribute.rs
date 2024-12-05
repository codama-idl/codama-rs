use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::BytesEncoding;

#[derive(Debug, PartialEq)]
pub struct StringAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub encoding: BytesEncoding,
}

impl<'a> StringAttribute<'a> {
    pub fn parse<T: TryInto<Self, Error = CodamaError>>(attr: T) -> CodamaResult<Self> {
        attr.try_into()
    }
}

impl<'a> TryFrom<&'a syn::Attribute> for StringAttribute<'a> {
    type Error = CodamaError;

    fn try_from(_attr: &'a syn::Attribute) -> CodamaResult<Self> {
        unimplemented!()
    }
}
