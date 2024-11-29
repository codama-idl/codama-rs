use crate::{as_derive_struct, get_type_params};
use codama_errors::CodamaResult;
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
    let item_generics = &input.generics;
    let item_type_params = get_type_params(&item_generics);

    Ok(quote! {
        impl #item_generics crate::TypeNodeTrait for #item_name #item_type_params{}
    })
}
