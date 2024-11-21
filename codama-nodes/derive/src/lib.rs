use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod into_enum;
mod node;
mod node_union;
mod type_node;
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

#[proc_macro_attribute]
pub fn type_node(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    type_node::expand_attribute_type_node(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(TypeNode)]
pub fn derive_type_node(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    type_node::expand_derive_type_node(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn node_union(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    node_union::expand_attribute_node_union(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(NodeUnion)]
pub fn derive_node_union(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    node_union::expand_derive_node_union(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(IntoEnum)]
pub fn derive_into_enum(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    into_enum::expand_derive_into_enum(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
