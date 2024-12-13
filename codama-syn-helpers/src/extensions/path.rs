use super::ToTokensExtension;
use codama_errors::CodamaResult;
use syn::{Path, PathArguments, PathSegment};

pub trait PathExtension {
    fn get_self(&self) -> &Path;

    /// Returns all segment idents as strings
    fn idents(&self) -> Vec<String> {
        let this = self.get_self();
        this.segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
    }

    /// Returns all segment idents joined by "::".
    /// E.g. for `a::b<B>::c::Option<T>` it returns `a::b::c::Option`.
    fn to_string(&self) -> String {
        self.idents().join("::")
    }

    /// Returns all segment idents joined by "::" except the last one.
    /// E.g. for `a::b<B>::c::Option<T>` it returns `a::b::c`.
    fn prefix(&self) -> String {
        let idents = self.idents();
        idents[..idents.len() - 1].join("::")
    }

    /// Returns the last segment.
    fn last(&self) -> &PathSegment {
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
        (this_prefix == prefix || this_prefix.is_empty()) && last == self.last_str()
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
            PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) => {
                args.iter().collect()
            }
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
        self.generic_types()
            .first()
            .copied()
            .ok_or_else(|| this.error("expected at least one generic type").into())
    }

    /// Returns the first generic type argument if there is exactly one.
    /// E.g. for `Vec<'a, T>` it returns `Ok(T)`.
    fn single_generic_type(&self) -> CodamaResult<&syn::Type> {
        let this = self.get_self();
        if self.generic_types().len() != 1 {
            return Err(this.error("expected a single generic type".to_string()).into());
        }
        self.first_generic_type()
    }
}

impl PathExtension for Path {
    fn get_self(&self) -> &Path {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idents() {
        let path: Path = syn::parse_quote! { std::option<Foo>::Option<String> };
        assert_eq!(path.idents(), vec!["std", "option", "Option"]);
    }

    #[test]
    fn to_string() {
        let path: Path = syn::parse_quote! { std::option<Foo>::Option<String> };
        assert_eq!(path.to_string(), "std::option::Option");
    }

    #[test]
    fn prefix() {
        let path: Path = syn::parse_quote! { std::option<Foo>::Option<String> };
        assert_eq!(path.prefix(), "std::option");
    }

    #[test]
    fn prefix_with_inner_generics() {
        let path: Path = syn::parse_quote! { a<A>::b<B>::c::Final };
        assert_eq!(path.prefix(), "a::b::c");
    }

    #[test]
    fn prefix_empty() {
        let path: Path = syn::parse_quote! { Foo };
        assert_eq!(path.prefix(), "");
    }

    #[test]
    fn is() {
        let path: Path = syn::parse_quote! { prefix::Foo<'a, T> };
        assert!(path.is("prefix::Foo"));
        assert!(!path.is("Foo"));
        assert!(!path.is("wrong::prefix::Foo"));
        assert!(!path.is("Bar"));

        let path: Path = syn::parse_quote! { Foo<T> };
        assert!(path.is("Foo"));
        assert!(path.is("prefix::Foo"));
        assert!(!path.is("Bar"));
    }

    #[test]
    fn is_strict() {
        let path: Path = syn::parse_quote! { prefix::Foo<'a, T> };
        assert!(path.is_strict("prefix::Foo"));
        assert!(!path.is_strict("Foo"));
        assert!(!path.is_strict("wrong::prefix::Foo"));
        assert!(!path.is_strict("Bar"));

        let path: Path = syn::parse_quote! { Foo<T> };
        assert!(path.is_strict("Foo"));
        assert!(!path.is_strict("prefix::Foo"));
        assert!(!path.is_strict("Bar"));
    }

    #[test]
    fn generic_arguments() {
        let path: Path = syn::parse_quote! { prefix::Foo<'a, T, U> };
        assert_eq!(path.generic_arguments().len(), 3);
    }

    #[test]
    fn generic_types() {
        let path: Path = syn::parse_quote! { prefix::Foo<'a, T, U> };
        assert_eq!(path.generic_types().len(), 2);
    }

    #[test]
    fn first_generic_type_ok() {
        let path: Path = syn::parse_quote! { prefix::Foo<'a, T, U> };
        assert!(matches!(path.first_generic_type(), Ok(syn::Type::Path(_))));
    }

    #[test]
    fn first_generic_type_err() {
        let path: Path = syn::parse_quote! { prefix::Foo<'a> };
        assert!(path.first_generic_type().is_err());
    }

    #[test]
    fn single_generic_type_ok() {
        let path: Path = syn::parse_quote! { Foo<'a, String> };
        assert!(matches!(path.single_generic_type(), Ok(syn::Type::Path(_))));
    }

    #[test]
    fn single_generic_type_err() {
        let path: Path = syn::parse_quote! { Foo<'a, String, u32> };
        assert!(path.single_generic_type().is_err());

        let path: Path = syn::parse_quote! { Foo<'a> };
        assert!(path.single_generic_type().is_err());
    }
}
