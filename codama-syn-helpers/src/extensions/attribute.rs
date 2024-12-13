use super::PathExtension;
use syn::{punctuated::Punctuated, Attribute};

pub trait AttributeExtension {
    fn get_self(&self) -> &Attribute;

    /// Parse all arguments as comma-separated types.
    fn parse_comma_args<T: syn::parse::Parse>(&self) -> syn::Result<Vec<T>> {
        self.get_self()
            .parse_args_with(Punctuated::<T, syn::Token![,]>::parse_terminated)
            .map(|metas| metas.into_iter().collect::<Vec<_>>())
    }

    /// Unwrap the feature flag from the attribute.
    /// E.g. `#[cfg_attr(feature = "some_feature", derive(Debug))]`
    /// becomes the syn::Meta defined as `derive(Debug)`.
    fn unfeatured(&self) -> Option<Attribute> {
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
        Some(Attribute {
            pound_token: this.pound_token,
            style: this.style,
            bracket_token: this.bracket_token,
            meta: inner_meta.clone(),
        })
    }
}

impl AttributeExtension for Attribute {
    fn get_self(&self) -> &Attribute {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::{quote, ToTokens};
    use syn::parse_quote;

    #[test]
    fn parse_comma_args_ok() {
        let attribute: Attribute = parse_quote! { #[foo(42, "bar")] };
        let args = attribute.parse_comma_args::<syn::Lit>().unwrap();
        assert_eq!(args.len(), 2);
    }

    #[test]
    fn parse_comma_args_err() {
        let attribute: Attribute = parse_quote! { #[foo] };
        let args = attribute.parse_comma_args::<syn::Path>();
        assert!(args.is_err());
    }

    #[test]
    fn unfeatured() {
        let attribute: Attribute =
            parse_quote! { #[cfg_attr(feature = "some_feature", derive(Debug))] };
        let unfeatured = attribute.unfeatured();
        assert_eq!(
            unfeatured.to_token_stream().to_string(),
            quote! { #[derive(Debug)] }.to_string()
        );
    }

    #[test]
    fn unfeatured_unchanged() {
        let attribute: Attribute = parse_quote! { #[derive(Debug)] };
        let unfeatured = attribute.unfeatured();
        assert_eq!(unfeatured, None);
    }
}
