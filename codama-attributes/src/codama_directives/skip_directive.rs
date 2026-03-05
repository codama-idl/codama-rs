use crate::{Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct SkipDirective;

impl SkipDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("skip")?;
        Ok(Self)
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a SkipDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Skip(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "skip".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a SkipDirective {
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
        let meta: Meta = syn::parse_quote! { skip };
        let directive = SkipDirective::parse(&meta).unwrap();
        assert_eq!(directive, SkipDirective);
    }
}
