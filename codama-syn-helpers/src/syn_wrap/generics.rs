pub struct GenericArguments<'a>(
    pub Option<&'a syn::punctuated::Punctuated<syn::GenericArgument, syn::Token![,]>>,
);

impl GenericArguments<'_> {
    /// Filters out all generic arguments that are not types.
    /// E.g. for `Option<'a, T, U>` it returns `[T, U]`.
    pub fn types(&self) -> Vec<&syn::Type> {
        match self.0 {
            Some(args) => args
                .iter()
                .filter_map(|arg| match arg {
                    syn::GenericArgument::Type(ty) => Some(ty),
                    _ => None,
                })
                .collect(),
            None => vec![],
        }
    }

    /// Returns the first genertic type argument if there is one.
    /// E.g. for `Vec<'a, T, U>` it returns `Some(T)`.
    pub fn first_type(&self) -> Option<&syn::Type> {
        self.types().first().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::quote;

    #[test]
    fn types() {
        let args = syn_build::parse::<syn::AngleBracketedGenericArguments>(quote! { <'a, T, U> });
        let args = GenericArguments(Some(&args.args));
        let types = args.types();
        assert_eq!(types.len(), 2);
    }

    #[test]
    fn first_type() {
        let args = syn_build::parse::<syn::AngleBracketedGenericArguments>(quote! { <'a, T, U> });
        let args = GenericArguments(Some(&args.args));
        let first_type = args.first_type();
        assert!(matches!(first_type, Some(syn::Type::Path(_))));
    }
}
