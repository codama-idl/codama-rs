use codama_errors::CodamaResult;
use codama_syn_helpers::extensions::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_registered_nodes(input: &syn::DeriveInput) -> CodamaResult<TokenStream> {
    let syn::DataEnum { variants, .. } = input.as_enum()?;
    let registered_enum_name = &input.ident;

    // Clone the enum.
    let mut standalone_enum = input.clone();

    // Remove the "Registered" prefix from the enum variants.
    standalone_enum.ident = syn::Ident::new(
        standalone_enum
            .ident
            .to_string()
            .trim_start_matches("Registered"),
        standalone_enum.ident.span(),
    );
    let standalone_enum_name = &standalone_enum.ident;
    let standalone_enum_name_str = standalone_enum.ident.to_string();

    // Get variants without the "registered" attribute.
    let is_standalone = |variant: &&syn::Variant| {
        variant
            .attrs
            .iter()
            .all(|attr| !attr.path().is_ident("registered"))
    };

    // Remove the variants with the "registered" attribute.
    match standalone_enum.data {
        syn::Data::Enum(ref mut data) => {
            data.variants = variants.iter().filter(is_standalone).cloned().collect();
        }
        _ => {
            return Err(syn::Error::new_spanned(input, "expected a enum").into());
        }
    }

    // Get the match patterns for both enums.
    let from_registered_patterns =
        variants
            .iter()
            .filter(is_standalone)
            .map(|syn::Variant { ident, .. }| {
                quote! {
                    #registered_enum_name::#ident(node) => Ok(Self::#ident(node.into())),
                }
            });
    let from_standalone_patterns =
        variants
            .iter()
            .filter(is_standalone)
            .map(|syn::Variant { ident, .. }| {
                quote! {
                    #standalone_enum_name::#ident(node) => Self::#ident(node.into()),
                }
            });

    // Add the standalone enum to the output.
    Ok(quote! {
        #standalone_enum

        impl TryFrom<#registered_enum_name> for #standalone_enum_name {
            type Error = codama_errors::CodamaError;

            fn try_from(value: #registered_enum_name) -> Result<Self, Self::Error> {
                match value {
                    #(#from_registered_patterns)*
                    _ => Err(codama_errors::CodamaError::InvalidNodeConversion {
                        from: value.kind().to_string(),
                        into: #standalone_enum_name_str.to_string(),
                    }),
                }
            }
        }

        impl From<#standalone_enum_name> for #registered_enum_name {
            fn from(value: #standalone_enum_name) -> Self {
                match value {
                    #(#from_standalone_patterns)*
                }
            }
        }

    })
}
