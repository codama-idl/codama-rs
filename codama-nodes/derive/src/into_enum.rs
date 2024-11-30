use codama_errors::CodamaResult;
use codama_syn_helpers::syn_traits::*;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{as_derive_enum, get_type_params};

pub fn expand_derive_into_enum(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    let data = as_derive_enum(input)?;
    let variants = &data.variants;
    let enum_name = &input.ident;
    let enum_generics = &input.generics;
    let enum_type_params = get_type_params(&enum_generics);

    // Generate an implementation block for each variant.
    let impl_blocks = variants
        .iter()
        .map(|variant| -> CodamaResult<TokenStream> {
            let variant_name = &variant.ident;
            let variant_type = match &variant.fields {
                syn::Fields::Unnamed(fields) => {
                    if fields.unnamed.len() != 1 {
                        return Err(syn::Error::new_spanned(
                            fields,
                            "expected a single field in the variant",
                        )
                        .into());
                    }
                    &fields.unnamed[0].ty
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "expected a single unnamed field in the variant",
                    )
                    .into());
                }
            };

            let variant_path = variant_type.as_path()?;
            let boxed_type = match (variant_path.is("Box"), variant_path.single_generic_type()) {
                (true, Ok(inner_type)) => Some(inner_type),
                _ => None,
            };
            let value = match boxed_type {
                Some(_) => quote! { value.into() },
                _ => quote! { value },
            };
            let input_type = boxed_type.unwrap_or(variant_type);

            Ok(quote! {
                impl #enum_generics From<#input_type> for #enum_name #enum_type_params {
                    fn from(value: #input_type) -> Self {
                        #enum_name::#variant_name(#value)
                    }
                }
            })
        })
        .collect::<CodamaResult<Vec<_>>>()?;

    // Render the macro output.
    Ok(quote! {
        #(#impl_blocks)*
    })
}
