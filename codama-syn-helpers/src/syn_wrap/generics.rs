use codama_errors::CodamaResult;

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

    /// Tries to return the first genertic type argument if there is one.
    /// E.g. for `Vec<'a, T, U>` it returns `Ok(T)`.
    pub fn try_first_type(&self) -> CodamaResult<&syn::Type> {
        self.types().first().copied().ok_or_else(|| {
            syn::Error::new_spanned(self.0, "expected at least one generic type").into()
        })
    }

    /// Returns the first genertic type argument or panics if there is none.
    /// E.g. for `Vec<'a, T, U>` it returns `T`.
    pub fn first_type(&self) -> &syn::Type {
        self.try_first_type().unwrap()
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
    fn first_type_ok() {
        let args = syn_build::parse::<syn::AngleBracketedGenericArguments>(quote! { <'a, T, U> });
        let args = GenericArguments(Some(&args.args));
        let first_type = args.try_first_type();
        assert!(matches!(first_type, Ok(syn::Type::Path(_))));
    }

    #[test]
    fn first_type_err() {
        let args = syn_build::parse::<syn::AngleBracketedGenericArguments>(quote! { <'a> });
        let args = GenericArguments(Some(&args.args));
        let first_type = args.try_first_type();
        assert!(matches!(first_type, Err(_)));
    }
}
