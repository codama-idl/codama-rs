use crate::{Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_nodes::PdaLinkNode;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct PdaDirective {
    pub pda: PdaLinkNode,
}

impl PdaDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let name = meta
            .assert_directive("pda")?
            .as_value()?
            .as_expr()?
            .as_string()?;
        Ok(Self {
            pda: PdaLinkNode::new(name),
        })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a PdaDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Pda(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "seed".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a PdaDirective {
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
        let meta: Meta = syn::parse_quote! { pda = "my_pda" };
        let directive = PdaDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            PdaDirective {
                pda: PdaLinkNode::new("my_pda"),
            }
        );
    }

    #[test]
    fn name_missing() {
        let meta: Meta = syn::parse_quote! { pda };
        let error = PdaDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "expected a value for that path: `pda = ...`"
        );
    }
}
