use crate::{Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct FixedSizeDirective {
    pub size: usize,
}

impl FixedSizeDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pv = meta.assert_directive("fixed_size")?.as_path_value()?;
        let size = pv.value.as_expr()?.as_literal_integer()?;
        Ok(Self { size })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a FixedSizeDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive {
            CodamaDirective::FixedSize(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "fixed_size".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a FixedSizeDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let meta: Meta = syn::parse_quote! { fixed_size = 42 };
        let directive = FixedSizeDirective::parse(&meta).unwrap();
        assert_eq!(directive, FixedSizeDirective { size: 42 });
    }
}
