use codama_attributes::{AttributeContext, CodamaAttribute};
use codama_errors::{CodamaError, CodamaResult};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

pub fn codama_attribute(attr: TokenStream, input: TokenStream) -> TokenStream {
    codama_attribute_impl(attr.into(), input.into())
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

fn codama_attribute_impl(attr: TokenStream2, input: TokenStream2) -> CodamaResult<TokenStream2> {
    let attr: syn::Attribute = syn::parse_quote! { #[codama(#attr)] };
    let item: syn::Item = syn::parse2(input.clone())?;
    let ctx: AttributeContext = (&item).into();
    CodamaAttribute::parse(&attr, &ctx)?;
    Ok(input)
}
