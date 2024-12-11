use std::fmt::Display;

pub trait ToTokens<T: quote::ToTokens> {
    fn get_self(&self) -> &T;

    fn error(&self, message: impl Display) -> syn::Error {
        syn::Error::new_spanned(self.get_self(), message)
    }
}

impl<T: quote::ToTokens> ToTokens<T> for T {
    fn get_self(&self) -> &T {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::quote;

    #[test]
    fn error() {
        let path: syn::Path = syn_build::parse(quote! { foo::bar });
        let error = path.error("expected a single segment");
        assert_eq!(error.to_string(), "expected a single segment");
    }
}
