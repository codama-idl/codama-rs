use codama_syn_helpers::{extensions::MetaListExtension, Meta};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn as_path(attr: TokenStream, input: TokenStream) -> TokenStream {
    handle(attr.into(), input.into(), |meta| meta.as_path().map(|_| ()))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn as_path_list(attr: TokenStream, input: TokenStream) -> TokenStream {
    handle(attr.into(), input.into(), |meta| {
        meta.as_path_list().map(|_| ())
    })
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}

#[proc_macro_attribute]
pub fn as_path_value(attr: TokenStream, input: TokenStream) -> TokenStream {
    handle(attr.into(), input.into(), |meta| {
        meta.as_path_value().map(|_| ())
    })
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}

#[proc_macro_attribute]
pub fn as_expr(attr: TokenStream, input: TokenStream) -> TokenStream {
    handle(attr.into(), input.into(), |meta| meta.as_expr().map(|_| ()))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn as_verbatim(attr: TokenStream, input: TokenStream) -> TokenStream {
    handle(attr.into(), input.into(), |meta| {
        meta.as_verbatim("expected verbatim").map(|_| ())
    })
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}

fn handle(
    attr: TokenStream2,
    input: TokenStream2,
    callback: fn(&Meta) -> syn::Result<()>,
) -> syn::Result<TokenStream2> {
    let attr: syn::Attribute = syn::parse_quote! { #[codama(#attr)] };
    attr.meta
        .require_list()?
        .parse_metas()?
        .iter()
        .map(callback)
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(input)
}
