use crate::{Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_nodes::CamelCaseString;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct NameDirective {
    pub name: CamelCaseString,
}

impl NameDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pv = meta.assert_directive("name")?.as_path_value()?;
        let name = pv.value.as_expr()?.as_string()?;
        Ok(Self { name: name.into() })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a NameDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive {
            CodamaDirective::Name(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "name".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a NameDirective {
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
        let meta: Meta = syn::parse_quote! { name = "banana" };
        let directive = NameDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            NameDirective {
                name: CamelCaseString::from("banana"),
            }
        );
    }
}
