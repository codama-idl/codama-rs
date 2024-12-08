use codama_errors::CodamaResult;
use syn::punctuated::Punctuated;

use super::Path;

pub trait Attribute {
    fn get_self(&self) -> &syn::Attribute;

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