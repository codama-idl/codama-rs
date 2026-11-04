use crate::{Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_nodes::Identifier;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct IdentifierDirective {
    pub identifier: Identifier,
}

impl IdentifierDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("identifier")?;
        Ok(Self {
            identifier: Identifier::try_from(meta.as_value()?.as_expr()?.as_string()?)
                .map_err(|e| meta.error(e))?,
        })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a IdentifierDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Identifier(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "identifier".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a IdentifierDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::test_utils::id;

    #[test]
    fn ok() {
        let meta: Meta = syn::parse_quote! { identifier = "banana" };
        let directive = IdentifierDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            IdentifierDirective {
                identifier: id("banana")
            }
        );
    }

    #[test]
    fn keeps_the_identifier_as_written() {
        for value in ["my_banana", "myBanana", "MyBanana"] {
            let meta: Meta = syn::parse_quote! { identifier = #value };
            let directive = IdentifierDirective::parse(&meta).unwrap();
            assert_eq!(directive.identifier, id(value));
        }
    }

    #[test]
    fn invalid_identifier() {
        for value in ["1st", "my banana", "my-banana"] {
            let meta: Meta = syn::parse_quote! { identifier = #value };
            let error = IdentifierDirective::parse(&meta).unwrap_err();
            assert_eq!(error.to_string(), format!("Invalid identifier: {value}"));
        }
    }
}
