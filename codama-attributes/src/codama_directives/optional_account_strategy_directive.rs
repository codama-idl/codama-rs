use crate::{utils::FromMeta, Attribute, CodamaAttribute, CodamaDirective};
use codama_errors::CodamaError;
use codama_nodes::InstructionOptionalAccountStrategy;
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OptionalAccountStrategyDirective {
    pub strategy: InstructionOptionalAccountStrategy,
}

impl OptionalAccountStrategyDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        Ok(Self {
            strategy: InstructionOptionalAccountStrategy::from_meta(
                meta.assert_directive("optional_account_strategy")?,
            )?,
        })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a OptionalAccountStrategyDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::OptionalAccountStrategy(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "optional_account_strategy".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a OptionalAccountStrategyDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_path_value() {
        let meta: Meta = syn::parse_quote! { optional_account_strategy = omitted };
        let directive = OptionalAccountStrategyDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            OptionalAccountStrategyDirective {
                strategy: InstructionOptionalAccountStrategy::Omitted,
            }
        );
    }
}
