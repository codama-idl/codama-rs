use codama_errors::CodamaResult;
use codama_syn_helpers::syn_traits::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_into_enum(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    let data = input.as_enum()?;
    let variants = &data.variants;
    let enum_name = &input.ident;
    let (pre_generics, post_generics) = input.generics.block_wrappers();

    // Generate an implementation block for each variant.
    let impl_blocks = variants
        .iter()
        .map(|variant| -> CodamaResult<TokenStream> {
            let variant_name = &variant.ident;
            let variant_type = &variant.fields.single_unnamed_field()?.ty;
            let boxed_type = variant_type.single_generic_type_from_path("Box").ok();
            let input_type = boxed_type.unwrap_or(variant_type);
            let value = match boxed_type {
                Some(_) => quote! { value.into() },
                _ => quote! { value },
            };

            Ok(quote! {
                impl #pre_generics From<#input_type> for #enum_name #post_generics {
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
