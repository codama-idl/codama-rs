use codama_errors::CodamaResult;
use codama_syn_helpers::extensions::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_attribute_nestable_type_node(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    input.as_struct()?;

    Ok(quote! {
        #[codama_nodes_derive::node]
        #[derive(codama_nodes_derive::NestableTypeNode)]
        #input
    })
}

pub fn expand_derive_nestable_type_node(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    input.as_struct()?;
    let item_name = &input.ident;

    Ok(quote! {
        impl crate::TypeNodeTrait for #item_name<crate::TypeNode> {
            fn into_type_node(self) -> crate::TypeNode {
                self.into()
            }
        }
    })
}
