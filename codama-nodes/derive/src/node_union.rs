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
    let (pre_generics, post_generics) = input.generics.block_wrappers();
    let mut item_generics_with_de = input.generics.clone();
    item_generics_with_de
        .params
        .insert(0, syn::parse_quote!('de));
    let (pre_generics_with_de, _) = item_generics_with_de.block_wrappers();

    let fallback_variant = variants.iter().find(|variant| {
        variant.attrs.iter().any(|attr| {
            let syn::Meta::Path(path) = &attr.meta else {
                return false;
            };
            path.is_ident("fallback")
        })
    });

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
        .filter(|variant| match fallback_variant {
            Some(fallback_variant) => *variant != fallback_variant,
            None => true,
        })
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

    let fallback_deserialize_pattern = match fallback_variant {
        Some(fallback_variant) => {
            let fallback_variant_name = &fallback_variant.ident;
            quote! {
                _ => Ok(#item_name::#fallback_variant_name(
                    serde_json::from_value(value).map_err(to_serde_error)?,
                )),
            }
        }
        None => {
            quote! { _ => Err(serde::de::Error::custom(format!(concat!("unknown kind {} for ", stringify!(#item_name)), kind))), }
        }
    };

    Ok(quote! {
        impl #pre_generics NodeUnionTrait for #item_name #post_generics {
            fn kind(&self) -> &'static str {
                match self {
                    #(#kind_patterns)*
                }
            }
        }

        impl #pre_generics serde::Serialize for #item_name #post_generics {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    #(#serialize_patterns)*
                }
            }
        }

        impl #pre_generics_with_de serde::Deserialize<'de> for #item_name #post_generics {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = serde_json::Value::deserialize(deserializer)?;
                let kind = value["kind"].as_str().ok_or_else(|| serde::de::Error::custom("missing kind"))?;
                let to_serde_error = |e: serde_json::Error| -> D::Error {
                    serde::de::Error::custom(format!("failed to deserialize AmountTypeNode: {}", e))
                };
                match kind {
                    #(#deserialize_patterns)*
                    #fallback_deserialize_pattern
                }
            }
        }
    })
}
