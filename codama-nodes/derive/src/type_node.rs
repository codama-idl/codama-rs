use crate::as_derive_struct;
use codama_errors::CodamaResult;
use codama_syn_helpers::syn_traits::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_attribute_type_node(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    as_derive_struct(&input)?;

    Ok(quote! {
        #[codama_nodes_derive::node]
        #[derive(codama_nodes_derive::TypeNode)]
        #input
    })
}

pub fn expand_derive_type_node(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    as_derive_struct(&input)?;
    let item_name = &input.ident;
    let (pre_generics, post_generics) = &input.generics.block_wrappers();

    Ok(quote! {
        impl #pre_generics crate::TypeNodeTrait for #item_name #post_generics{}
    })
}
