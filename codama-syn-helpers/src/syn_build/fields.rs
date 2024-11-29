use codama_errors::CodamaResult;
use proc_macro2::TokenStream;
use quote::quote;

/// E.g. `pub foo: u32`
pub fn try_named_field(tt: TokenStream) -> CodamaResult<syn::Field> {
    let ast = syn::parse2::<syn::ItemStruct>(quote! { struct Foo { #tt } })?;
    let field = match &ast.fields {
        syn::Fields::Named(f) => f.named.first().cloned(),
        _ => None,
    };
    match field {
        Some(f) => Ok(f),
        None => Err(syn::Error::new_spanned(tt, "expected a named field").into()),
    }
}

/// E.g. `pub foo: u32`
pub fn named_field(tt: TokenStream) -> syn::Field {
    try_named_field(tt).unwrap()
}

/// E.g. `pub u32`
pub fn try_unnamed_field(tt: TokenStream) -> CodamaResult<syn::Field> {
    let ast = syn::parse2::<syn::ItemStruct>(quote! { struct Foo (#tt); })?;
    let field = match &ast.fields {
        syn::Fields::Unnamed(f) => f.unnamed.first().cloned(),
        _ => None,
    };
    match field {
        Some(f) => Ok(f),
        None => Err(syn::Error::new_spanned(tt, "expected an unnamed field").into()),
    }
}

/// E.g. `pub u32`
pub fn unnamed_field(tt: TokenStream) -> syn::Field {
    try_unnamed_field(tt).unwrap()
}

#[cfg(test)]
mod tests {
    use codama_errors::CodamaError;

    use super::*;

    #[test]
    fn named_field_ok() {
        let result = try_named_field(quote! { pub foo: u32 });
        assert!(matches!(result, Ok(syn::Field { ident: Some(_), .. })));
    }

    #[test]
    fn named_field_err() {
        let result = try_named_field(quote! { u32 });
        assert!(matches!(result, Err(CodamaError::Compilation(_))));
    }

    #[test]
    fn unnamed_field_ok() {
        let result = try_unnamed_field(quote! { u32 });
        assert!(matches!(result, Ok(syn::Field { ident: None, .. })));
    }

    #[test]
    fn unnamed_field_err() {
        let result = try_unnamed_field(quote! { pub foo: u32  });
        assert!(matches!(result, Err(CodamaError::Compilation(_))));
    }
}
