use codama_errors::CodamaResult;
use syn::punctuated::Punctuated;

use super::Path;

pub trait Attribute {
    fn get_self(&self) -> &syn::Attribute;

    /// Ensure the attribute meta is a path.
    fn as_path(&self) -> CodamaResult<&syn::Path> {
        self.get_self().meta.require_path_only().map_err(Into::into)
    }

    /// Ensure the attribute meta is a path.
    fn as_list(&self) -> CodamaResult<&syn::MetaList> {
        self.get_self().meta.require_list().map_err(Into::into)
    }

    /// Ensure the attribute meta is a path.
    fn as_name_value(&self) -> CodamaResult<&syn::MetaNameValue> {
        self.get_self()
            .meta
            .require_name_value()
            .map_err(Into::into)
    }

    /// Parse all arguments as comma-separated types.
    fn parse_comma_args<T: syn::parse::Parse>(&self) -> CodamaResult<Vec<T>> {
        self.get_self()
            .parse_args_with(Punctuated::<T, syn::Token![,]>::parse_terminated)
            .map(|metas| metas.into_iter().collect::<Vec<_>>())
            .map_err(Into::into)
    }

    /// Unwrap the feature flag from the attribute.
    /// E.g. `#[cfg_attr(feature = "some_feature", derive(Debug))]`
    /// becomes the syn::Meta defined as `derive(Debug)`.
    fn unfeatured(&self) -> Option<syn::Attribute> {
        let this = self.get_self();
        if !this.path().is_strict("cfg_attr") {
            return None;
        }
        let metas = this.parse_comma_args::<syn::Meta>().ok()?;
        let [feature, inner_meta] = metas.as_slice() else {
            return None;
        };
        match feature {
            syn::Meta::NameValue(m) if m.path.is_strict("feature") => (),
            _ => return None,
        }
        Some(syn::Attribute {
            pound_token: this.pound_token,
            style: this.style,
            bracket_token: this.bracket_token,
            meta: inner_meta.clone(),
        })
    }
}

impl Attribute for syn::Attribute {
    fn get_self(&self) -> &syn::Attribute {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::{quote, ToTokens};

    #[test]
    fn as_path_ok() {
        let attribute = syn_build::attribute(quote! { #[serde] });
        assert!(matches!(attribute.as_path(), Ok(_)));
    }

    #[test]
    fn as_path_err() {
        let attribute = syn_build::attribute(quote! { #[derive(Debug)] });
        assert!(matches!(attribute.as_path(), Err(_)));
    }

    #[test]
    fn as_list_ok() {
        let attribute = syn_build::attribute(quote! { #[derive(Debug)] });
        assert!(matches!(attribute.as_list(), Ok(_)));
    }

    #[test]
    fn as_list_err() {
        let attribute = syn_build::attribute(quote! { #[serde] });
        assert!(matches!(attribute.as_list(), Err(_)));
    }

    #[test]
    fn as_name_value_ok() {
        let attribute = syn_build::attribute(quote! { #[foo = 42] });
        assert!(matches!(attribute.as_name_value(), Ok(_)));
    }

    #[test]
    fn as_name_value_err() {
        let attribute = syn_build::attribute(quote! { #[serde] });
        assert!(matches!(attribute.as_name_value(), Err(_)));
    }

    #[test]
    fn parse_comma_args_ok() {
        let attribute = syn_build::attribute(quote! { #[foo(42, "bar")] });
        let args = attribute.parse_comma_args::<syn::Lit>().unwrap();
        assert_eq!(args.len(), 2);
    }

    #[test]
    fn parse_comma_args_err() {
        let attribute = syn_build::attribute(quote! { #[foo] });
        let args = attribute.parse_comma_args::<syn::Path>();
        assert!(matches!(args, Err(_)));
    }

    #[test]
    fn unfeatured() {
        let attribute =
            syn_build::attribute(quote! { #[cfg_attr(feature = "some_feature", derive(Debug))] });
        let unfeatured = attribute.unfeatured();
        assert_eq!(
            unfeatured.to_token_stream().to_string(),
            quote! { #[derive(Debug)] }.to_string()
        );
    }

    #[test]
    fn unfeatured_unchanged() {
        let attribute = syn_build::attribute(quote! { #[derive(Debug)] });
        let unfeatured = attribute.unfeatured();
        assert_eq!(unfeatured, None);
    }
}
