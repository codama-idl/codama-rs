use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{Endian, NumberFormat};

#[derive(Debug, PartialEq)]
pub struct NumberAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub endian: Option<Endian>,
    pub format: Option<NumberFormat>,
}

impl<'a> NumberAttribute<'a> {
    pub fn parse<T: TryInto<Self, Error = CodamaError>>(attr: T) -> CodamaResult<Self> {
        attr.try_into()
    }
}

impl<'a> TryFrom<&'a syn::Attribute> for NumberAttribute<'a> {
    type Error = CodamaError;

    fn try_from(_attr: &'a syn::Attribute) -> CodamaResult<Self> {
        unimplemented!()
    }
}
