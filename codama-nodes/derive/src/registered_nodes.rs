use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_registered_nodes(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    // Clone the enum.
    let mut standalone_enum = input.clone();

    // Remove the "Registered" prefix from the enum variants.
    standalone_enum.ident = syn::Ident::new(
        &standalone_enum
            .ident
            .to_string()
            .trim_start_matches("Registered")
            .to_string(),
        standalone_enum.ident.span(),
    );

    // Remove the variants with the "registered" attribute.
    match standalone_enum.data {
        syn::Data::Enum(ref mut data) => {
            data.variants = data
                .variants
                .iter()
                .filter(|variant| {
                    variant
                        .attrs
                        .iter()
                        .all(|attr| !attr.path().is_ident("registered"))
                })
                .cloned()
                .collect();
        }
        _ => {
            return Err(syn::Error::new_spanned(input, "expected a enum").into());
        }
    }

    // Add the standalone enum to the output.
    Ok(quote! {
        #standalone_enum
    })
}
