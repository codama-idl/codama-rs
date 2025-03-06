use crate::Attribute;
use codama_errors::CodamaError;

#[derive(Debug, PartialEq)]
pub struct UnsupportedAttribute<'a> {
    pub ast: &'a syn::Attribute,
}

impl<'a> UnsupportedAttribute<'a> {
    pub fn new(ast: &'a syn::Attribute) -> Self {
        Self { ast }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a UnsupportedAttribute<'a> {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        match attribute {
            Attribute::Unsupported(a) => Ok(a),
            _ => Err(CodamaError::InvalidAttribute {
                expected: "unsupported".to_string(),
                actual: attribute.name(),
            }),
        }
    }
}
