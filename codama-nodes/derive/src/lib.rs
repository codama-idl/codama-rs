use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IntoEnum)]
pub fn derive(input: TokenStream) -> TokenStream {
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
    let enum_type_params = enum_generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                quote! { #ident }
            }
            syn::GenericParam::Lifetime(lifetime) => {
                let lifetime = &lifetime.lifetime;
                quote! { #lifetime }
            }
            syn::GenericParam::Const(const_param) => {
                let ident = &const_param.ident;
                quote! { #ident }
            }
        })
        .collect::<Vec<_>>();
    let enum_type_params = if enum_type_params.is_empty() {
        quote! {}
    } else {
        quote! { <#(#enum_type_params),*> }
    };

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

// Identify the inner type of a type — e.g. `Box<T>` -> `T`.
fn unwrap_inner_type<'a>(ty: &'a syn::Type, ident: &'a str) -> Option<&'a syn::Type> {
    // Get the path of the type. — e.g. `a::b::c::Option`.
    let syn::Type::Path(syn::TypePath { path, .. }) = ty else {
        return None;
    };

    // Only match single-segment paths whose ident is expected.
    if !is_single_path(path, ident) {
        return None;
    };
    let segment = &path.segments[0];

    // Get the generic arguments of the segment.
    let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        args: generic_args,
        ..
    }) = &segment.arguments
    else {
        return None;
    };

    // Only match types with a single generic argument.
    if generic_args.len() != 1 {
        return None;
    };

    // Ensure the generic argument is also a type.
    let syn::GenericArgument::Type(ty) = generic_args.first().unwrap() else {
        return None;
    };

    // Return the inner type.
    Some(ty)
}

fn is_single_path(path: &syn::Path, ident: &str) -> bool {
    path.segments.len() == 1 && path.segments[0].ident == ident
}
