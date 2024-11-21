use crate::{as_derive_struct, get_type_params, lowercase_first_letter};
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_attribute_node(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    as_derive_struct(&input)?;
    let item_name = &input.ident;
    let kind = lowercase_first_letter(&item_name.to_string());

    Ok(quote! {
        #[derive(codama_nodes_derive::Node, core::fmt::Debug, core::cmp::PartialEq, core::clone::Clone, serde::Serialize, serde::Deserialize)]
        #[serde(tag = "kind", rename = #kind)]
        #[serde(rename_all = "camelCase")]
        #input
    })
}

pub fn expand_derive_node(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    as_derive_struct(&input)?;
    let item_name = &input.ident;
    let item_generics = &input.generics;
    let item_type_params = get_type_params(&item_generics);
    let kind = lowercase_first_letter(&item_name.to_string());

    Ok(quote! {
        impl #item_generics crate::NodeTrait for #item_name #item_type_params{
            const KIND: &'static str = #kind;
        }
    })
}
