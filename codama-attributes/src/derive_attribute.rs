use crate::Attribute;
use codama_errors::CodamaError;
use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct DeriveAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub derives: Vec<syn::Path>,
}

impl<'a> DeriveAttribute<'a> {
    pub fn parse(ast: &'a syn::Attribute) -> syn::Result<Self> {
        let unfeatured = ast.unfeatured();
        let effective = unfeatured.as_ref().unwrap_or(ast);
        Self::parse_from(ast, effective)
    }

    /// Parse a derive attribute using the effective attribute for content extraction.
    /// `ast` is stored as the original attribute reference (for error spans).
    /// `effective` is used to parse the actual derive list.
    pub fn parse_from(ast: &'a syn::Attribute, effective: &syn::Attribute) -> syn::Result<Self> {
        let list = effective.meta.require_list()?;
        if !list.path.is_strict("derive") {
            return Err(list.path.error("expected #[derive(...)]"));
        };

        let derives = list.parse_comma_args::<syn::Path>()?;
        Ok(Self { ast, derives })
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a DeriveAttribute<'a> {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        match attribute {
            Attribute::Derive(a) => Ok(a),
            _ => Err(CodamaError::InvalidAttribute {
                expected: "derive".to_string(),
                actual: attribute.name(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_derive_attribute() {
        let ast = parse_quote! { #[derive(Debug, PartialEq)] };
        let attribute = DeriveAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(
            attribute.derives,
            [(parse_quote! { Debug }), (parse_quote! { PartialEq }),]
        );
    }

    #[test]
    fn test_feature_gated_derive_attribute() {
        let ast = parse_quote! { #[cfg_attr(feature = "some_feature", derive(Debug, PartialEq))] };
        let attribute = DeriveAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(
            attribute.derives,
            [(parse_quote! { Debug }), (parse_quote! { PartialEq })]
        );
    }
}
