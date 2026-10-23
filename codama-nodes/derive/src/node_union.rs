use crate::lowercase_first_letter;
use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_syn_helpers::extensions::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_attribute_node_union(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    input.as_enum()?;

    Ok(quote! {
        #[derive(codama_nodes_derive::NodeUnion, derive_more::From, core::fmt::Debug, core::cmp::PartialEq, core::clone::Clone)]
        #input
    })
}

pub fn expand_derive_node_union(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    let data = input.as_enum()?;
    let variants = &data.variants;
    let item_name = &input.ident;

    let kind_patterns = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            #item_name::#variant_name(node) => node.kind(),
        }
    });

    let serialize_patterns = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            #item_name::#variant_name(node) => node.serialize(serializer),
        }
    });

    let deserialize_patterns = variants
        .iter()
        .map(|variant| -> CodamaResult<TokenStream> {
            let variant_name = &variant.ident;
            let variant_type = &variant.fields.single_unnamed_field()?.ty;
            let variant_type = variant_type
                .single_generic_type_from_path("Box")
                .unwrap_or(variant_type);
            let kind = lowercase_first_letter(&variant_type.as_path()?.last_str());

            Ok(quote! {
                #kind => Ok(#item_name::#variant_name(
                    serde_json::from_value(value).map_err(to_serde_error)?,
                )),
            })
        })
        .collect_and_combine_errors()?;

    Ok(quote! {
        impl crate::NodeUnionTrait for #item_name {}

        impl crate::HasKind for #item_name {
            fn kind(&self) -> &'static str {
                match self {
                    #(#kind_patterns)*
                }
            }
        }

        impl serde::Serialize for #item_name {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    #(#serialize_patterns)*
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for #item_name {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = serde_json::Value::deserialize(deserializer)?;
                let kind = value["kind"].as_str().ok_or_else(|| serde::de::Error::custom("missing kind"))?;
                let to_serde_error = |e: serde_json::Error| -> D::Error {
                    serde::de::Error::custom(format!(concat!("failed to deserialize ", stringify!(#item_name), ": {}"), e))
                };
                match kind {
                    #(#deserialize_patterns)*
                    _ => Err(serde::de::Error::custom(format!(concat!("unknown kind {} for ", stringify!(#item_name)), kind))),
                }
            }
        }
    })
}
