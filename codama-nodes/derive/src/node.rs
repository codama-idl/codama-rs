use crate::{as_derive_struct, lowercase_first_letter};
use codama_errors::CodamaResult;
use codama_syn_helpers::syn_traits::Generics;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_attribute_node(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
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

pub fn expand_derive_node(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    as_derive_struct(&input)?;
    let item_name = &input.ident;
    let (pre_generics, post_generics) = &input.generics.block_wrappers();
    let kind = lowercase_first_letter(&item_name.to_string());

    Ok(quote! {
        impl #pre_generics crate::NodeTrait for #item_name #post_generics{
            const KIND: &'static str = #kind;
        }
    })
}
