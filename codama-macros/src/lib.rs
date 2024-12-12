use codama_attributes::CodamaAttribute;
use codama_errors::{CodamaError, CodamaResult};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn codama(attr: TokenStream, input: TokenStream) -> TokenStream {
    codama_attribute(attr.into(), input.into())
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

fn codama_attribute(attr: TokenStream2, input: TokenStream2) -> CodamaResult<TokenStream2> {
    let attr: syn::Attribute = syn::parse_quote! { #[codama(#attr)] };
    CodamaAttribute::try_from(&attr)?;
    Ok(input.into())
}
