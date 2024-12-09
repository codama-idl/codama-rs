use codama_attributes::NodeAttribute;
use codama_errors::{CodamaError, CodamaResult};
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn node(attr: TokenStream, input: TokenStream) -> TokenStream {
    node_attribute(attr.into(), input.into())
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

fn node_attribute(
    attr: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> CodamaResult<proc_macro2::TokenStream> {
    let attr: syn::Attribute = syn::parse_quote! { #[node(#attr)] };
    NodeAttribute::parse(&attr)?;
    Ok(input.into())
}
