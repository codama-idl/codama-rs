use codama_errors::{CodamaError, CodamaResult};
use codama_koroks::CrateKorok;
use codama_stores::CrateStore;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn codama_derive(input: TokenStream) -> TokenStream {
    codama_derive_impl(input.into())
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

fn codama_derive_impl(input: TokenStream2) -> CodamaResult<TokenStream2> {
    let store = CrateStore::hydrate(input)?;
    CrateKorok::parse(&store)?;
    Ok(quote! {})
}
