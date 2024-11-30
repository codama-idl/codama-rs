use codama_errors::CodamaResult;

pub trait Path {
    fn get_self(&self) -> &syn::Path;

    /// Returns all segment idents joined by "::" except the last one.
    /// E.g. for `a::b<B>::c::Option<T>` it returns `a::b::c`.
    fn prefix(&self) -> String {
        let this = self.get_self();
        this.segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()[..this.segments.len() - 1]
            .join("::")
    }

    /// Returns the last segment.
    fn last(&self) -> &syn::PathSegment {
        self.get_self().segments.last().unwrap()
    }

    /// Returns the ident of the last segment as a string.
    fn last_str(&self) -> String {
        self.last().ident.to_string()
    }

    /// Returns true if the path is equal to the given path including or excluding the prefix.
    fn is(&self, path: &str) -> bool {
        let mut segments = path.split("::").collect::<Vec<_>>();
        let last = segments.pop().unwrap();
        let prefix = segments.join("::");
        let this_prefix = self.prefix();
        (this_prefix == prefix || this_prefix == "") && last == self.last_str()
    }

    /// Returns true if the path is equal to the given path including the prefix.
    fn is_strict(&self, path: &str) -> bool {
        let mut segments = path.split("::").collect::<Vec<_>>();
        let last = segments.pop().unwrap();
        let prefix = segments.join("::");
        prefix == self.prefix() && last == self.last_str()
    }

    /// Returns the generic arguments of the last segment.
    /// E.g. for `a::b::c::Option<'a, T, U>` it returns `GenericArguments(Some(['a, T, U]))`.
    /// E.g. for `a::b::c::u32` it returns `GenericArguments(None)`.
    fn generic_arguments(&self) -> Vec<&syn::GenericArgument> {
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
    fn generic_types(&self) -> Vec<&syn::Type> {
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
    fn first_generic_type(&self) -> CodamaResult<&syn::Type> {
        let this = self.get_self();
        self.generic_types().first().copied().ok_or_else(|| {
            syn::Error::new_spanned(this, "expected at least one generic type").into()
        })
    }

    /// Returns the first generic type argument if there is exactly one.
    /// E.g. for `Vec<'a, T>` it returns `Ok(T)`.
    fn single_generic_type(&self) -> CodamaResult<&syn::Type> {
        let this = self.get_self();
        if self.generic_types().len() != 1 {
            return Err(
                syn::Error::new_spanned(this, format!("expected a single generic type")).into(),
            );
        }
        self.first_generic_type()
    }
}

impl Path for syn::Path {
    fn get_self(&self) -> &syn::Path {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::quote;

    #[test]
    fn prefix() {
        let path: syn::Path = syn_build::parse(quote! { std::option::Option<String> });
        assert_eq!(path.prefix(), "std::option");
    }

    #[test]
    fn prefix_with_inner_generics() {
        let path: syn::Path = syn_build::parse(quote! { a<A>::b<B>::c::Final });
        assert_eq!(path.prefix(), "a::b::c");
    }

    #[test]
    fn prefix_empty() {
        let path: syn::Path = syn_build::parse(quote! { Foo });
        assert_eq!(path.prefix(), "");
    }

    #[test]
    fn is() {
        let path: syn::Path = syn_build::parse(quote! { prefix::Foo<'a, T> });
        assert_eq!(path.is("prefix::Foo"), true);
        assert_eq!(path.is("Foo"), false);
        assert_eq!(path.is("wrong::prefix::Foo"), false);
        assert_eq!(path.is("Bar"), false);

        let path: syn::Path = syn_build::parse(quote! { Foo<T> });
        assert_eq!(path.is("Foo"), true);
        assert_eq!(path.is("prefix::Foo"), true);
        assert_eq!(path.is("Bar"), false);
    }

    #[test]
    fn is_strict() {
        let path: syn::Path = syn_build::parse(quote! { prefix::Foo<'a, T> });
        assert_eq!(path.is_strict("prefix::Foo"), true);
        assert_eq!(path.is_strict("Foo"), false);
        assert_eq!(path.is_strict("wrong::prefix::Foo"), false);
        assert_eq!(path.is_strict("Bar"), false);

        let path: syn::Path = syn_build::parse(quote! { Foo<T> });
        assert_eq!(path.is_strict("Foo"), true);
        assert_eq!(path.is_strict("prefix::Foo"), false);
        assert_eq!(path.is_strict("Bar"), false);
    }

    #[test]
    fn generic_arguments() {
        let path: syn::Path = syn_build::parse(quote! { prefix::Foo<'a, T, U> });
        assert_eq!(path.generic_arguments().len(), 3);
    }

    #[test]
    fn generic_types() {
        let path: syn::Path = syn_build::parse(quote! { prefix::Foo<'a, T, U> });
        assert_eq!(path.generic_types().len(), 2);
    }

    #[test]
    fn first_generic_type_ok() {
        let path: syn::Path = syn_build::parse(quote! { prefix::Foo<'a, T, U> });
        assert!(matches!(path.first_generic_type(), Ok(syn::Type::Path(_))));
    }

    #[test]
    fn first_generic_type_err() {
        let path: syn::Path = syn_build::parse(quote! { prefix::Foo<'a> });
        assert!(matches!(path.first_generic_type(), Err(_)));
    }

    #[test]
    fn single_generic_type_ok() {
        let path: syn::Path = syn_build::parse(quote! { Foo<'a, String> });
        assert!(matches!(path.single_generic_type(), Ok(syn::Type::Path(_))));
    }

    #[test]
    fn single_generic_type_err() {
        let path: syn::Path = syn_build::parse(quote! { Foo<'a, String, u32> });
        assert!(matches!(path.single_generic_type(), Err(_)));

        let path: syn::Path = syn_build::parse(quote! { Foo<'a> });
        assert!(matches!(path.single_generic_type(), Err(_)));
    }
}
