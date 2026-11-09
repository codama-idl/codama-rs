use codama_errors::{CodamaResult, IteratorCombineErrors};
use codama_syn_helpers::extensions::*;
use proc_macro2::TokenStream;
use quote::quote;

/// Converts a node union to and from the flat `crate::Node`, and borrows it as a `crate::NodeRef`.
/// Each variant wraps a node struct, possibly boxed, which converts through its own generated impls.
pub fn expand_derive_node_conversions(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    let data = input.as_enum()?;
    let item_name = &input.ident;
    let item_name_str = item_name.to_string();

    let variants = data
        .variants
        .iter()
        .map(|variant| -> CodamaResult<(&syn::Ident, &syn::Type, bool)> {
            let variant_type = &variant.fields.single_unnamed_field()?.ty;
            Ok(match variant_type.single_generic_type_from_path("Box") {
                Ok(inner) => (&variant.ident, inner, true),
                Err(_) => (&variant.ident, variant_type, false),
            })
        })
        .collect_and_combine_errors()?;

    let into_node_patterns = variants.iter().map(|(name, _, boxed)| match boxed {
        true => quote! { #item_name::#name(node) => crate::Node::from(*node), },
        false => quote! { #item_name::#name(node) => crate::Node::from(node), },
    });

    let into_node_ref_patterns = variants.iter().map(|(name, _, boxed)| match boxed {
        true => quote! { #item_name::#name(node) => crate::NodeRef::from(&**node), },
        false => quote! { #item_name::#name(node) => crate::NodeRef::from(node), },
    });

    // A `FooBarNode` struct is the `Node::FooBar` variant, so a type off that convention fails to compile.
    let from_node_patterns = variants
        .iter()
        .map(|(name, ty, boxed)| -> CodamaResult<TokenStream> {
            let type_name = ty.as_path()?.last_str();
            let variant = syn::Ident::new(
                type_name.strip_suffix("Node").unwrap_or(&type_name),
                proc_macro2::Span::call_site(),
            );
            Ok(match boxed {
                true => {
                    quote! { crate::Node::#variant(node) => Ok(#item_name::#name(Box::new(node))), }
                }
                false => quote! { crate::Node::#variant(node) => Ok(#item_name::#name(node)), },
            })
        })
        .collect_and_combine_errors()?;

    Ok(quote! {
        impl From<#item_name> for crate::Node {
            fn from(node: #item_name) -> Self {
                match node {
                    #(#into_node_patterns)*
                }
            }
        }

        impl<'a> From<&'a #item_name> for crate::NodeRef<'a> {
            fn from(node: &'a #item_name) -> Self {
                match node {
                    #(#into_node_ref_patterns)*
                }
            }
        }

        impl TryFrom<crate::Node> for #item_name {
            type Error = codama_errors::CodamaError;

            fn try_from(node: crate::Node) -> core::result::Result<Self, Self::Error> {
                match node {
                    #(#from_node_patterns)*
                    node => Err(codama_errors::CodamaError::InvalidNodeConversion {
                        from: crate::HasKind::kind(&node).to_string(),
                        into: #item_name_str.to_string(),
                    }),
                }
            }
        }
    })
}
