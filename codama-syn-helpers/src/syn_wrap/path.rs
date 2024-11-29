use codama_errors::CodamaResult;
use quote::ToTokens;
use std::ops::Deref;

pub struct Path<'a>(pub &'a syn::Path);

impl<'a> Path<'a> {
    /// Returns all segment idents joined by "::" except the last one.
    /// E.g. for `a::b<B>::c::Option<T>` it returns `a::b::c`.
    pub fn prefix(&self) -> String {
        self.0
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()[..self.0.segments.len() - 1]
            .join("::")
    }

    /// Returns the last segment.
    pub fn last(&self) -> &'a syn::PathSegment {
        self.0.segments.last().unwrap()
    }

    /// Returns the ident of the last segment as a string.
    pub fn last_str(&self) -> String {
        self.last().ident.to_string()
    }

    /// Returns true if the path is equal to the given path including or excluding the prefix.
    pub fn is(&self, path: &str) -> bool {
        let mut segments = path.split("::").collect::<Vec<_>>();
        let last = segments.pop().unwrap();
        let prefix = segments.join("::");
        (prefix == self.prefix() || prefix == "") && last == self.last_str()
    }

    /// Returns true if the path is equal to the given path including the prefix.
    pub fn is_strict(&self, path: &str) -> bool {
        let mut segments = path.split("::").collect::<Vec<_>>();
        let last = segments.pop().unwrap();
        let prefix = segments.join("::");
        prefix == self.prefix() && last == self.last_str()
    }

    /// Returns the generic arguments of the last segment.
    /// E.g. for `a::b::c::Option<'a, T, U>` it returns `GenericArguments(Some(['a, T, U]))`.
    /// E.g. for `a::b::c::u32` it returns `GenericArguments(None)`.
    pub fn generic_arguments(&self) -> Vec<&'a syn::GenericArgument> {
        match &self.last().arguments {
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => args.iter().collect(),
            _ => vec![],
        }
    }

    /// Filters out all generic arguments that are not types.
    /// E.g. for `Option<'a, T, U>` it returns `[T, U]`.
    pub fn generic_types(&self) -> Vec<&'a syn::Type> {
        self.generic_arguments()
            .iter()
            .filter_map(|arg| match arg {
                syn::GenericArgument::Type(ty) => Some(ty),
                _ => None,
            })
            .collect()
    }

    /// Returns the first generic type argument if there is one.
    /// E.g. for `Vec<'a, T, U>` it returns `Ok(T)`.
    pub fn first_generic_type(&self) -> CodamaResult<&'a syn::Type> {
        self.generic_types().first().copied().ok_or_else(|| {
            syn::Error::new_spanned(self.0, "expected at least one generic type").into()
        })
    }

    /// Returns the first generic type argument if there is exactly one.
    /// E.g. for `Vec<'a, T>` it returns `Ok(T)`.
    pub fn single_generic_type(&self) -> CodamaResult<&'a syn::Type> {
        if self.generic_types().len() != 1 {
            return Err(
                syn::Error::new_spanned(self.0, format!("expected a single generic type")).into(),
            );
        }
        self.first_generic_type()
    }
}

impl Deref for Path<'_> {
    type Target = syn::Path;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl ToTokens for Path<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::quote;

    #[test]
    fn prefix() {
        let path = syn_build::parse(quote! { std::option::Option<String> });
        let result = Path(&path).prefix();
        assert_eq!(result, "std::option");
    }

    #[test]
    fn prefix_with_inner_generics() {
        let result = Path(&syn_build::parse(quote! { a<A>::b<B>::c::Final })).prefix();
        assert_eq!(result, "a::b::c");
    }

    #[test]
    fn prefix_empty() {
        let result = Path(&syn_build::parse(quote! { Foo })).prefix();
        assert_eq!(result, "");
    }

    #[test]
    fn is() {
        let path = syn_build::parse(quote! { prefix::Foo<'a, T> });
        assert_eq!(Path(&path).is("prefix::Foo"), true);
        assert_eq!(Path(&path).is("Foo"), true);
        assert_eq!(Path(&path).is("wrong::prefix::Foo"), false);
        assert_eq!(Path(&path).is("Bar"), false);

        let path = syn_build::parse(quote! { Foo<T> });
        assert_eq!(Path(&path).is("Foo"), true);
        assert_eq!(Path(&path).is("prefix::Foo"), false);
        assert_eq!(Path(&path).is("Bar"), false);
    }

    #[test]
    fn is_strict() {
        let path = syn_build::parse(quote! { prefix::Foo<'a, T> });
        assert_eq!(Path(&path).is_strict("prefix::Foo"), true);
        assert_eq!(Path(&path).is_strict("Foo"), false);
        assert_eq!(Path(&path).is_strict("wrong::prefix::Foo"), false);
        assert_eq!(Path(&path).is_strict("Bar"), false);

        let path = syn_build::parse(quote! { Foo<T> });
        assert_eq!(Path(&path).is_strict("Foo"), true);
        assert_eq!(Path(&path).is_strict("prefix::Foo"), false);
        assert_eq!(Path(&path).is_strict("Bar"), false);
    }

    #[test]
    fn generic_arguments() {
        let path = syn_build::parse(quote! { prefix::Foo<'a, T, U> });
        assert_eq!(Path(&path).generic_arguments().len(), 3);
    }

    #[test]
    fn generic_types() {
        let path = syn_build::parse(quote! { prefix::Foo<'a, T, U> });
        assert_eq!(Path(&path).generic_types().len(), 2);
    }

    #[test]
    fn first_generic_type_ok() {
        let path = syn_build::parse(quote! { prefix::Foo<'a, T, U> });
        assert!(matches!(
            Path(&path).first_generic_type(),
            Ok(syn::Type::Path(_))
        ));
    }

    #[test]
    fn first_generic_type_err() {
        let path = syn_build::parse(quote! { prefix::Foo<'a> });
        assert!(matches!(Path(&path).first_generic_type(), Err(_)));
    }

    #[test]
    fn single_generic_type_ok() {
        let path = syn_build::parse(quote! { Foo<'a, String> });
        assert!(matches!(
            Path(&path).single_generic_type(),
            Ok(syn::Type::Path(_))
        ));
    }

    #[test]
    fn single_generic_type_err() {
        let path = syn_build::parse(quote! { Foo<'a, String, u32> });
        assert!(matches!(Path(&path).single_generic_type(), Err(_)));

        let path = syn_build::parse(quote! { Foo<'a> });
        assert!(matches!(Path(&path).single_generic_type(), Err(_)));
    }
}
