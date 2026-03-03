use crate::{utils::SetOnce, Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct ProgramDirective {
    pub name: String,
    pub address: String,
}

impl ProgramDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("program")?.as_path_list()?;

        let mut name = SetOnce::<String>::new("name");
        let mut address = SetOnce::<String>::new("address");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "name" => name.set(meta.as_value()?.as_expr()?.as_string()?, meta),
            "address" => address.set(meta.as_value()?.as_expr()?.as_string()?, meta),
            _ => Err(meta.error("unrecognized attribute")),
        })?;

        Ok(Self {
            name: name.take(meta)?,
            address: address.take(meta)?,
        })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a ProgramDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Program(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "program".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a ProgramDirective {
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
        let meta: Meta = syn::parse_quote! { program(name = "associatedToken", address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL") };
        let directive = ProgramDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            ProgramDirective {
                name: "associatedToken".to_string(),
                address: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".to_string(),
            }
        );
    }

    #[test]
    fn name_missing() {
        let meta: Meta =
            syn::parse_quote! { program(address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL") };
        let error = ProgramDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "name is missing");
    }

    #[test]
    fn address_missing() {
        let meta: Meta = syn::parse_quote! { program(name = "associatedToken") };
        let error = ProgramDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "address is missing");
    }
}
