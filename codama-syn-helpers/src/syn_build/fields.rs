use codama_errors::CodamaResult;
use proc_macro2::TokenStream;
use quote::quote;

/// E.g. `{ pub foo: u32, bar: String }` or `(pub u32, String)`
pub fn try_fields(tt: TokenStream) -> CodamaResult<syn::Fields> {
    let ast = match syn::parse2::<syn::ItemStruct>(quote! { struct Foo #tt }) {
        Ok(ast) => ast,
        Err(_) => syn::parse2::<syn::ItemStruct>(quote! { struct Foo #tt; })?,
    };
    Ok(ast.fields)
}

/// E.g. `{ pub foo: u32, bar: String }` or `(pub u32, String)`
pub fn fields(tt: TokenStream) -> syn::Fields {
    try_fields(tt).unwrap()
}

#[cfg(test)]
mod tests {
    use codama_errors::CodamaError;

    use super::*;

    #[test]
    fn named_fields_ok() {
        let result = try_fields(quote! { { pub foo: u32, bar: String } });
        assert!(matches!(result, Ok(syn::Fields::Named(_))));
    }

    #[test]
    fn unnamed_fields_ok() {
        let result = try_fields(quote! { (pub u32, String) });
        assert!(matches!(result, Ok(syn::Fields::Unnamed(_))));
    }

    #[test]
    fn unit_fields_ok() {
        let result = try_fields(quote! {});
        assert!(matches!(result, Ok(syn::Fields::Unit)));
    }

    #[test]
    fn fields_err() {
        let result = try_fields(quote! { struct Foo {} });
        assert!(matches!(result, Err(CodamaError::Compilation(_))));
    }
}
