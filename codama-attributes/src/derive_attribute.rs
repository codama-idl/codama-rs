use codama_errors::{CodamaError, CodamaResult};
use codama_syn_helpers::syn_traits::*;
use syn::punctuated::Punctuated;

#[derive(Debug, PartialEq)]
pub struct DeriveAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub derives: Vec<syn::Path>,
}

impl<'a> DeriveAttribute<'a> {
    pub fn parse<T: TryInto<Self, Error = CodamaError>>(attr: T) -> CodamaResult<Self> {
        attr.try_into()
    }
}

impl<'a> TryFrom<&'a syn::Attribute> for DeriveAttribute<'a> {
    type Error = CodamaError;

    fn try_from(attr: &'a syn::Attribute) -> CodamaResult<Self> {
        // TODO: unfeature the attribute.
        // E.g. `#[cfg_attr(feature = "some_feature", derive(Debug))]`
        // becomes `#[derive(Debug)]`

        let list = attr.as_list()?;
        if !list.path.is_strict("derive") {
            return Err(syn::Error::new_spanned(&list.path, "expected #[derive(...)]").into());
        };

        let derives = attr
            .parse_args_with(Punctuated::<syn::Path, syn::Token![,]>::parse_terminated)?
            .into_iter()
            .collect();

        Ok(Self { ast: attr, derives })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_syn_helpers::syn_build;
    use quote::quote;

    #[test]
    fn test_derive_attribute() {
        let ast = syn_build::attribute(quote! { #[derive(Debug, PartialEq)] });
        let attribute = DeriveAttribute::parse(&ast).unwrap();

        assert_eq!(attribute.ast, &ast);
        assert_eq!(
            attribute.derives,
            [
                syn_build::parse(quote! { Debug }),
                syn_build::parse(quote! { PartialEq }),
            ]
        );
    }
}
