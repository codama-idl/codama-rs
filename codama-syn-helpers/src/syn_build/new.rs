use codama_errors::CodamaResult;
use proc_macro2::TokenStream;

/// Parses any token stream into a syn::parse::Parse type.
pub fn try_new<T: syn::parse::Parse>(tt: TokenStream) -> CodamaResult<T> {
    syn::parse2::<T>(tt).map_err(|e| e.into())
}

/// Parses any token stream into a syn::parse::Parse type.
pub fn new<T: syn::parse::Parse>(tt: TokenStream) -> T {
    try_new(tt).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_errors::CodamaError;
    use quote::quote;

    #[test]
    fn new_ok() {
        let result = try_new::<syn::Type>(quote! { std::option::Option<String> });
        assert!(matches!(result, Ok(syn::Type::Path(_))));
    }

    #[test]
    fn new_err() {
        let result = try_new::<syn::Type>(quote! { struct Foo {} });
        assert!(matches!(result, Err(CodamaError::Compilation(_))));
    }
}
