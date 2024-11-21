use proc_macro2::TokenStream;
use quote::quote;

use crate::{as_derive_enum, get_type_params, unwrap_inner_type};

pub fn expand_derive_into_enum(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let data = as_derive_enum(input)?;
    let variants = &data.variants;
    let enum_name = &input.ident;
    let enum_generics = &input.generics;
    let enum_type_params = get_type_params(&enum_generics);

    // Generate an implementation block for each variant.
    let impl_blocks = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_type = match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    return syn::Error::new_spanned(
                        fields,
                        "expected a single field in the variant",
                    )
                    .to_compile_error();
                }
                &fields.unnamed[0].ty
            }
            _ => {
                return syn::Error::new_spanned(
                    variant,
                    "expected a single unnamed field in the variant",
                )
                .to_compile_error();
            }
        };

        let boxed_type = unwrap_inner_type(variant_type, "Box");
        let input_type = boxed_type.unwrap_or(variant_type);
        let value = if boxed_type.is_some() {
            quote! { value.into() }
        } else {
            quote! { value }
        };

        quote! {
            impl #enum_generics From<#input_type> for #enum_name #enum_type_params {
                fn from(value: #input_type) -> Self {
                    #enum_name::#variant_name(#value)
                }
            }
        }
    });

    // Render the macro output.
    Ok(quote! {
        #(#impl_blocks)*
    })
}
