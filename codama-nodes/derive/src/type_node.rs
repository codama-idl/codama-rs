use codama_errors::CodamaResult;
use codama_syn_helpers::syn_traits::*;
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

    // The item name ident without the last 8 characters (for "TypeNode").
    let variant_name = item_name.to_string();
    let variant_name = syn::Ident::new(&variant_name[..variant_name.len() - 8], item_name.span());

    // Is the variant boxed?
    let is_boxed = [
        "Array",
        "Map",
        "Option",
        "RemainderOption",
        "Set",
        "ZeroableOption",
    ]
    .contains(&variant_name.to_string().as_str());
    let ok_result = match is_boxed {
        true => quote! { Ok(*node) },
        false => quote! { Ok(node) },
    };

    Ok(quote! {
        impl #pre_generics crate::TypeNodeTrait for #item_name #post_generics{
            fn from_type_node(node: crate::TypeNode) -> codama_errors::CodamaResult<Self> {
                use crate::NodeTrait;
                match node {
                    crate::TypeNode::#variant_name(node) => #ok_result,
                    _ => Err(codama_errors::CodamaError::InvalidNodeConversion {
                        from: "TypeNode".into(),
                        into: #item_name::KIND.into(),
                    }),
                }
            }
        }
    })
}
