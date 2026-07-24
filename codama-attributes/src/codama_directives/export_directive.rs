use crate::{Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct ExportDirective {
    pub derive: String,
}

impl ExportDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pl = meta
            .assert_directive("export")?
            .as_path_list()
            .map_err(|_| {
                meta.error("export expects a single derive name — e.g. `export(CodamaType)`")
            })?;
        let metas = pl.parse_metas()?;
        let [arg] = metas.as_slice() else {
            return Err(
                meta.error("export expects a single derive name — e.g. `export(CodamaType)`")
            );
        };
        let derive = arg.as_path()?.last_str();
        match derive.as_str() {
            "CodamaAccount" | "CodamaAccounts" | "CodamaErrors" | "CodamaEvent"
            | "CodamaEvents" | "CodamaInstruction" | "CodamaInstructions" | "CodamaPda"
            | "CodamaType" => Ok(Self { derive }),
            _ => Err(arg.error("unrecognized codama derive")),
        }
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a ExportDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Export(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "export".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a ExportDirective {
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
        let meta: Meta = syn::parse_quote! { export(CodamaType) };
        let directive = ExportDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            ExportDirective {
                derive: "CodamaType".to_string()
            }
        );
    }

    #[test]
    fn fail_without_derive_name() {
        let meta: Meta = syn::parse_quote! { export };
        let error = ExportDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "export expects a single derive name — e.g. `export(CodamaType)`"
        );
    }

    #[test]
    fn fail_with_unrecognized_derive() {
        let meta: Meta = syn::parse_quote! { export(NotADerive) };
        let error = ExportDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized codama derive");
    }
}
