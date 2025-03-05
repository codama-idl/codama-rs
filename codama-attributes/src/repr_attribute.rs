use codama_syn_helpers::extensions::*;

#[derive(Debug, PartialEq)]
pub struct ReprAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub metas: Vec<syn::Meta>,
}

impl<'a> ReprAttribute<'a> {
    pub fn parse(ast: &'a syn::Attribute) -> syn::Result<Self> {
        // Check if the attribute is feature-gated.
        let unfeatured = ast.unfeatured();
        let attr = unfeatured.as_ref().unwrap_or(ast);

        // Check if the attribute is a #[repr(...)] attribute.
        let list = attr.meta.require_list()?;
        if !list.path.is_strict("repr") {
            return Err(list.path.error("expected #[repr(...)]"));
        };

        // Parse the list of metas.
        let metas = list.parse_comma_args::<syn::Meta>()?;
        Ok(Self { ast, metas })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_repr_attribute() {
        let ast = parse_quote! { #[repr(u32, align(4))] };
        let attribute = ReprAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(
            attribute.metas,
            [(parse_quote! { u32 }), (parse_quote! { align(4) })]
        );
    }

    #[test]
    fn test_feature_gated_repr_attribute() {
        let ast = parse_quote! { #[cfg_attr(feature = "some_feature", repr(u32, align(4)))] };
        let attribute = ReprAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(
            attribute.metas,
            [(parse_quote! { u32 }), (parse_quote! { align(4) })]
        );
    }
}
