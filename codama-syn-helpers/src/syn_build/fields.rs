use codama_errors::CodamaResult;
use proc_macro2::TokenStream;
use quote::quote;

/// E.g. `pub foo: u32` or `pub u32`
pub fn try_field(tt: TokenStream) -> CodamaResult<syn::Field> {
    let ast = match syn::parse2::<syn::ItemStruct>(quote! { struct Foo { #tt } }) {
        Ok(ast) => ast,
        Err(_) => syn::parse2::<syn::ItemStruct>(quote! {struct Foo (#tt); })?,
    };
    let field = match &ast.fields {
        syn::Fields::Named(f) => f.named.first().cloned(),
        syn::Fields::Unnamed(f) => f.unnamed.first().cloned(),
        _ => None,
    };
    match field {
        Some(f) => Ok(f),
        None => Err(syn::Error::new_spanned(tt, "expected a field").into()),
    }
}

/// E.g. `pub foo: u32` or `pub u32`
pub fn field(tt: TokenStream) -> syn::Field {
    try_field(tt).unwrap()
}

#[cfg(test)]
mod tests {
    use codama_errors::CodamaError;

    use super::*;

    #[test]
    fn named_field_ok() {
        let result = try_field(quote! { pub foo: u32 });
        assert!(matches!(result, Ok(syn::Field { ident: Some(_), .. })));
    }

    #[test]
    fn unnamed_field_ok() {
        let result = try_field(quote! { u32 });
        assert!(matches!(result, Ok(syn::Field { ident: None, .. })));
    }

    #[test]
    fn field_err() {
        let result = try_field(quote! { struct Foo {} });
        assert!(matches!(result, Err(CodamaError::Compilation(_))));
    }
}
