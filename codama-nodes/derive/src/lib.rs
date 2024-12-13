use codama_errors::CodamaError;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod into_enum;
mod nestable_type_node;
mod node;
mod node_union;
mod registered_nodes;
mod type_node;
mod utils;
use utils::*;

#[proc_macro_attribute]
pub fn node(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    node::expand_attribute_node(&input)
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

#[proc_macro_derive(Node)]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    node::expand_derive_node(&input)
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn type_node(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    type_node::expand_attribute_type_node(&input)
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

#[proc_macro_derive(TypeNode)]
pub fn derive_type_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    type_node::expand_derive_type_node(&input)
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn nestable_type_node(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    nestable_type_node::expand_attribute_nestable_type_node(&input)
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

#[proc_macro_derive(NestableTypeNode)]
pub fn derive_nestable_type_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    nestable_type_node::expand_derive_nestable_type_node(&input)
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn node_union(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    node_union::expand_attribute_node_union(&input)
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

#[proc_macro_derive(NodeUnion, attributes(fallback))]
pub fn derive_node_union(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    node_union::expand_derive_node_union(&input)
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

#[proc_macro_derive(IntoEnum)]
pub fn derive_into_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    into_enum::expand_derive_into_enum(&input)
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}

#[proc_macro_derive(RegisteredNodes, attributes(registered))]
pub fn derive_registered_nodes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    registered_nodes::expand_derive_registered_nodes(&input)
        .unwrap_or_else(CodamaError::into_compile_error)
        .into()
}
