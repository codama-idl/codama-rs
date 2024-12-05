use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::TypeNode;

#[derive(Debug, PartialEq)]
pub struct TypeAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub node: TypeNode,
}

impl<'a> TypeAttribute<'a> {
    pub fn parse<T: TryInto<Self, Error = CodamaError>>(attr: T) -> CodamaResult<Self> {
        attr.try_into()
    }
}

impl<'a> TryFrom<&'a syn::Attribute> for TypeAttribute<'a> {
    type Error = CodamaError;

    fn try_from(_attr: &'a syn::Attribute) -> CodamaResult<Self> {
        unimplemented!()
    }
}
