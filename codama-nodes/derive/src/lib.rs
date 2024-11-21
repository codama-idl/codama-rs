use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod into_enum;
mod node;
mod utils;
use utils::*;

#[proc_macro_attribute]
pub fn node(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    node::expand_attribute_node(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Node)]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    node::expand_derive_node(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
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
    let mut input = parse_macro_input!(input as DeriveInput);
    into_enum::expand_derive_into_enum(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
