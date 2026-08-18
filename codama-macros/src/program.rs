use codama_attributes::CodamaProgramMacro;
use codama_errors::{CodamaError, CodamaResult};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn codama_program(input: TokenStream) -> TokenStream {
    codama_program_impl(input.into())
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

fn codama_program_impl(input: TokenStream2) -> CodamaResult<TokenStream2> {
    CodamaProgramMacro::parse(input)?;
    Ok(quote! {})
}
