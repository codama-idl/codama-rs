use super::{new, try_new};
use codama_errors::CodamaResult;
use proc_macro2::TokenStream;

/// E.g. `std::collections::HashSet<String>`, `[u8; 4]`, etc.
pub fn try_type(tt: TokenStream) -> CodamaResult<syn::Type> {
    try_new(tt)
}

/// E.g. `std::collections::HashSet<String>`, `[u8; 4]`, etc.
pub fn r#type(tt: TokenStream) -> syn::Type {
    new(tt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_errors::CodamaError;
    use quote::quote;

    #[test]
    fn type_ok() {
        let result = try_type(quote! { std::option::Option<String> });
        assert!(matches!(result, Ok(syn::Type::Path(_))));
    }

    #[test]
    fn type_err() {
        let result = try_type(quote! { struct Foo {} });
        assert!(matches!(result, Err(CodamaError::Compilation(_))));
    }
}
