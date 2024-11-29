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
