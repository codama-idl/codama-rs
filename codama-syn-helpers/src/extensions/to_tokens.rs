use quote::ToTokens;
use std::fmt::Display;

pub trait ToTokensExtension<T: ToTokens> {
    fn get_self(&self) -> &T;

    fn error(&self, message: impl Display) -> syn::Error {
        syn::Error::new_spanned(self.get_self(), message)
    }
}

impl<T: ToTokens> ToTokensExtension<T> for T {
    fn get_self(&self) -> &T {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error() {
        let path: syn::Path = syn::parse_quote! { foo::bar };
        let error = path.error("expected a single segment");
        assert_eq!(error.to_string(), "expected a single segment");
    }
}
