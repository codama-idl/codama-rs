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

    /// Unwrap all inner attributes from a feature-gated `cfg_attr`.
    /// E.g. `#[cfg_attr(feature = "x", derive(Debug), codama(...))]`
    /// returns `vec![#[derive(Debug)], #[codama(...)]]`.
    /// Returns an empty Vec if not a feature-gated `cfg_attr`.
    fn unfeatured_all(&self) -> Vec<Attribute> {
        let this = self.get_self();
        if !this.path().is_strict("cfg_attr") {
            return vec![];
        }
        let metas = match this.parse_comma_args::<syn::Meta>() {
            Ok(m) => m,
            Err(_) => return vec![],
        };
        if metas.len() < 2 {
            return vec![];
        }
        // First item should be the feature condition
        match &metas[0] {
            syn::Meta::NameValue(m) if m.path.is_strict("feature") => (),
            _ => return vec![],
        }
        // Rest are the inner attributes
        metas[1..]
            .iter()
            .map(|meta| Attribute {
                pound_token: this.pound_token,
                style: this.style,
                bracket_token: this.bracket_token,
                meta: meta.clone(),
            })
            .collect()
    }

    /// Unwrap the feature flag from the attribute.
    /// E.g. `#[cfg_attr(feature = "some_feature", derive(Debug))]`
    /// becomes the syn::Attribute defined as `#[derive(Debug)]`.
    /// If the `cfg_attr` contains multiple inner attributes, returns the first.
    fn unfeatured(&self) -> Option<Attribute> {
        self.unfeatured_all().into_iter().next()
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
    fn unfeatured_single() {
        let attribute: Attribute =
            parse_quote! { #[cfg_attr(feature = "some_feature", derive(Debug))] };
        let unfeatured = attribute.unfeatured();
        assert_eq!(
            unfeatured.to_token_stream().to_string(),
            quote! { #[derive(Debug)] }.to_string()
        );
    }

    #[test]
    fn unfeatured_multiple_returns_first() {
        let attribute: Attribute =
            parse_quote! { #[cfg_attr(feature = "x", derive(Debug), codama(foo))] };
        let unfeatured = attribute.unfeatured();
        assert_eq!(
            unfeatured.to_token_stream().to_string(),
            quote! { #[derive(Debug)] }.to_string()
        );
    }

    #[test]
    fn unfeatured_not_cfg_attr() {
        let attribute: Attribute = parse_quote! { #[derive(Debug)] };
        let unfeatured = attribute.unfeatured();
        assert_eq!(unfeatured, None);
    }

    #[test]
    fn unfeatured_all_single() {
        let attribute: Attribute = parse_quote! { #[cfg_attr(feature = "x", derive(Debug))] };
        let all = attribute.unfeatured_all();
        assert_eq!(all.len(), 1);
        assert_eq!(
            all[0].to_token_stream().to_string(),
            quote! { #[derive(Debug)] }.to_string()
        );
    }

    #[test]
    fn unfeatured_all_multiple() {
        let attribute: Attribute = parse_quote! {
            #[cfg_attr(feature = "codama", codama(account(name = "stake")), codama(account(name = "auth")))]
        };
        let all = attribute.unfeatured_all();
        assert_eq!(all.len(), 2);
        assert_eq!(
            all[0].to_token_stream().to_string(),
            quote! { #[codama(account(name = "stake"))] }.to_string()
        );
        assert_eq!(
            all[1].to_token_stream().to_string(),
            quote! { #[codama(account(name = "auth"))] }.to_string()
        );
    }

    #[test]
    fn unfeatured_all_not_cfg_attr() {
        let attribute: Attribute = parse_quote! { #[derive(Debug)] };
        let all = attribute.unfeatured_all();
        assert!(all.is_empty());
    }

    #[test]
    fn unfeatured_all_mixed_attrs() {
        let attribute: Attribute = parse_quote! {
            #[cfg_attr(feature = "x", derive(Debug), repr(u8), codama(foo))]
        };
        let all = attribute.unfeatured_all();
        assert_eq!(all.len(), 3);
        assert_eq!(
            all[0].to_token_stream().to_string(),
            quote! { #[derive(Debug)] }.to_string()
        );
        assert_eq!(
            all[1].to_token_stream().to_string(),
            quote! { #[repr(u8)] }.to_string()
        );
        assert_eq!(
            all[2].to_token_stream().to_string(),
            quote! { #[codama(foo)] }.to_string()
        );
    }
}
