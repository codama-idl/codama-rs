use codama_attributes::CodamaAttribute;
use codama_errors::{CodamaError, CodamaResult};
use codama_koroks::CrateKorok;
use codama_stores::CrateStore;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro_derive(CodamaAccount, attributes(codama))]
pub fn codama_account_derive(input: TokenStream) -> TokenStream {
    codama_derive(input.into())
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

#[proc_macro_derive(CodamaInstruction, attributes(codama))]
pub fn codama_instruction_derive(input: TokenStream) -> TokenStream {
    codama_derive(input.into())
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

#[proc_macro_derive(CodamaType, attributes(codama))]
pub fn codama_type_derive(input: TokenStream) -> TokenStream {
    codama_derive(input.into())
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

fn codama_derive(input: TokenStream2) -> CodamaResult<TokenStream2> {
    let store = CrateStore::hydrate(input)?;
    CrateKorok::parse(&store)?;
    Ok(quote! {})
}

#[proc_macro_attribute]
pub fn codama(attr: TokenStream, input: TokenStream) -> TokenStream {
    codama_attribute(attr.into(), input.into())
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

fn codama_attribute(attr: TokenStream2, input: TokenStream2) -> CodamaResult<TokenStream2> {
    let attr: syn::Attribute = syn::parse_quote! { #[codama(#attr)] };
    CodamaAttribute::try_from(&attr)?;
    Ok(input)
}
