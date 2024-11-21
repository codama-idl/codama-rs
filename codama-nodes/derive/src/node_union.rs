use crate::as_derive_enum;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_attribute_node_union(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    as_derive_enum(&input)?;

    Ok(quote! {
        #[derive(codama_nodes_derive::IntoEnum, core::fmt::Debug, core::cmp::PartialEq, core::clone::Clone, serde::Serialize, serde::Deserialize)]
        #[serde(untagged)]
        #input
    })
}
