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
        // Check if the attribute is feature-gated.
        let unfeatured = ast.unfeatured();
        let attr = unfeatured.as_ref().unwrap_or(ast);

        // Check if the attribute is a #[derive(...)] attribute.
        let list = attr.meta.require_list()?;
        if !list.path.is_strict("derive") {
            return Err(list.path.error("expected #[derive(...)]"));
        };

        // Parse the list of derives.
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
