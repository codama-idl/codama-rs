use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct DeriveAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub derives: Vec<syn::Path>,
}

impl<'a> TryFrom<&'a syn::Attribute> for DeriveAttribute<'a> {
    type Error = syn::Error;

    fn try_from(ast: &'a syn::Attribute) -> syn::Result<Self> {
        // Check if the attribute is feature-gated.
        let unfeatured = ast.unfeatured();
        let attr = unfeatured.as_ref().unwrap_or(ast);

        // Check if the attribute is a #[derive(...)] attribute.
        let list = attr.meta.require_list()?;
        if !list.path.is_strict("derive") {
            return Err(list.path.error("expected #[derive(...)]").into());
        };

        // Parse the list of derives.
        let derives = list.parse_comma_args::<syn::Path>()?;
        Ok(Self { ast, derives })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_derive_attribute() {
        let ast = parse_quote! { #[derive(Debug, PartialEq)] };
        let attribute = DeriveAttribute::try_from(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(
            attribute.derives,
            [(parse_quote! { Debug }), (parse_quote! { PartialEq }),]
        );
    }

    #[test]
    fn test_feature_gated_derive_attribute() {
        let ast = parse_quote! { #[cfg_attr(feature = "some_feature", derive(Debug, PartialEq))] };
        let attribute = DeriveAttribute::try_from(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(
            attribute.derives,
            [(parse_quote! { Debug }), (parse_quote! { PartialEq })]
        );
    }
}
