use super::GenericArguments;

pub struct Path<'a>(pub &'a syn::Path);

impl Path<'_> {
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
    pub fn last(&self) -> &syn::PathSegment {
        self.0.segments.last().unwrap()
    }

    /// Returns the ident of the last segment as a string.
    pub fn last_indent(&self) -> String {
        self.last().ident.to_string()
    }

    /// Returns the generic arguments of the last segment.
    /// E.g. for `a::b::c::Option<'a, T, U>` it returns `GenericArguments(Some(['a, T, U]))`.
    /// E.g. for `a::b::c::u32` it returns `GenericArguments(None)`.
    pub fn generic_arguments(&self) -> GenericArguments {
        match &self.last().arguments {
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => GenericArguments(Some(args)),
            _ => GenericArguments(None),
        }
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
        let path = syn_build::parse(quote! { a<A>::b<B>::c::Final });
        let result = Path(&path).prefix();
        assert_eq!(result, "a::b::c");
    }

    #[test]
    fn prefix_empty() {
        let path = syn_build::parse(quote! { Foo });
        let result = Path(&path).prefix();
        assert_eq!(result, "");
    }

    #[test]
    fn generic_arguments() {
        let path = syn_build::parse(quote! { prefix::Foo<'a, T, U> });
        let path = Path(&path);
        let result = path.generic_arguments();
        assert!(matches!(result, GenericArguments(Some(_))));
    }
}
