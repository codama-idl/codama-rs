use crate::{as_derive_enum, get_type_params, lowercase_first_letter};
use codama_errors::CodamaResult;
use codama_syn_helpers::syn_traits::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_attribute_node_union(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    as_derive_enum(&input)?;

    Ok(quote! {
        #[derive(codama_nodes_derive::NodeUnion, codama_nodes_derive::IntoEnum, core::fmt::Debug, core::cmp::PartialEq, core::clone::Clone)]
        #input
    })
}

pub fn expand_derive_node_union(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    let data = as_derive_enum(&input)?;
    let variants = &data.variants;
    let item_name = &input.ident;
    let item_generics = &input.generics;
    let item_type_params = get_type_params(&item_generics);
    let mut item_generics_with_de = item_generics.clone();
    item_generics_with_de
        .params
        .insert(0, syn::parse_quote!('de));

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
        .collect::<CodamaResult<Vec<_>>>()?;

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
        impl #item_generics NodeUnionTrait for #item_name #item_type_params {
            fn kind(&self) -> &'static str {
                match self {
                    #(#kind_patterns)*
                }
            }
        }

        impl #item_generics serde::Serialize for #item_name #item_type_params {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    #(#serialize_patterns)*
                }
            }
        }

        impl #item_generics_with_de serde::Deserialize<'de> for #item_name #item_type_params {
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
