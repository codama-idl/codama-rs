use codama_errors::CodamaResult;
use codama_syn_helpers::extensions::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_attribute_type_node(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    input.as_struct()?;

    Ok(quote! {
        #[codama_nodes_derive::node]
        #[derive(codama_nodes_derive::TypeNode)]
        #input
    })
}

pub fn expand_derive_type_node(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    input.as_struct()?;
    let item_name = &input.ident;
    let (pre_generics, post_generics) = &input.generics.block_wrappers();

    // The item name ident without the last 8 characters (for "TypeNode" or "LinkNode").
    let variant_name = item_name.to_string();
    let variant_name = syn::Ident::new(&variant_name[..variant_name.len() - 8], item_name.span());

    Ok(quote! {
        impl #pre_generics crate::TypeNodeTrait for #item_name #post_generics{
            fn try_from_type_node(node: crate::TypeNode) -> codama_errors::CodamaResult<Self> {
                use crate::{HasKind, NodeTrait};
                match node {
                    crate::TypeNode::#variant_name(node) => Ok(node),
                    _ => Err(codama_errors::CodamaError::InvalidNodeConversion {
                        from: node.kind().to_string(),
                        into: Self::KIND.into(),
                    }),
                }
            }
            fn into_type_node(self) -> crate::TypeNode {
                self.into()
            }
        }
    })
}
