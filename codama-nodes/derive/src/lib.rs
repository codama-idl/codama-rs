use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod utils;
use utils::*;

#[proc_macro_derive(Node)]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let syn::Data::Struct(_) = input.data else {
        // Return a compile error if the attribute is not on a struct.
        return syn::Error::new_spanned(input, "expected a struct")
            .to_compile_error()
            .into();
    };

    let item_name = &input.ident;
    let item_generics = input.generics;
    let item_type_params = get_type_params(&item_generics);
    let kind = lowercase_first_letter(&item_name.to_string());

    // Render the macro output.
    quote! {
        impl #item_generics crate::NodeTrait for #item_name #item_type_params{
            const KIND: &'static str = #kind;
        }
    }
    .into()
}

#[proc_macro_derive(TypeNode)]
pub fn derive_type_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let syn::Data::Struct(_) = input.data else {
        // Return a compile error if the attribute is not on a struct.
        return syn::Error::new_spanned(input, "expected a struct")
            .to_compile_error()
            .into();
    };

    let item_name = &input.ident;
    let item_generics = input.generics;
    let item_type_params = get_type_params(&item_generics);

    // Render the macro output.
    quote! {
        impl #item_generics crate::TypeNodeTrait for #item_name #item_type_params{}
    }
    .into()
}

#[proc_macro_derive(IntoEnum)]
pub fn derive_into_enum(input: TokenStream) -> TokenStream {
    // Derive an AST from the input token stream.
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the variants from the enum.
    let variants = match input.data {
        syn::Data::Enum(syn::DataEnum { ref variants, .. }) => variants,
        _ => {
            // Return a compile error if the derive is not on a struct.
            return syn::Error::new_spanned(input, "expected an enum")
                .to_compile_error()
                .into();
        }
    };

    // Extract the name of the enum.
    let enum_name = input.ident;

    // Extract the full generic definition of the enum — e.g. `<T: SomeTrait, U>`.
    let enum_generics = input.generics;

    // Extract only the type parameters of the enum — e.g. `<T, U>`.
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
    quote! {
        #(#impl_blocks)*
    }
    .into()
}
