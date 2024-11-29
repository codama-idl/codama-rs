use crate::{as_derive_enum, get_type_params, lowercase_first_letter, unwrap_inner_type};
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_attribute_node_union(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    as_derive_enum(&input)?;

    Ok(quote! {
        #[derive(codama_nodes_derive::NodeUnion, codama_nodes_derive::IntoEnum, core::fmt::Debug, core::cmp::PartialEq, core::clone::Clone)]
        #input
    })
}

pub fn expand_derive_node_union(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
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
        .map(|variant| {
            let variant_name = &variant.ident;
            let syn::Fields::Unnamed(fields) = &variant.fields else {
                return Err(syn::Error::new_spanned(
                    variant,
                    "expected a single unnamed field in the variant",
                ));
            };
            let node_type = &(fields.unnamed.first()).unwrap().ty;
            let node_type = match unwrap_inner_type(&node_type, "Box") {
                Some(inner_type) => inner_type,
                None => node_type,
            };
            let syn::Type::Path(node_type_path) = node_type else {
                return Err(syn::Error::new_spanned(node_type, "expected a path type"));
            };
            let node_type_ident = &node_type_path.path.segments.first().unwrap().ident;
            let kind = lowercase_first_letter(&node_type_ident.to_string());

            Ok(quote! {
                #kind => Ok(#item_name::#variant_name(
                    serde_json::from_value(value).map_err(to_serde_error)?,
                )),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

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
