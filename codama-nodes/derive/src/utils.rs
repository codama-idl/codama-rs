use codama_errors::CodamaResult;
use quote::quote;

// Ensure the derive input is a struct and return the data.
pub fn as_derive_struct(input: &syn::DeriveInput) -> CodamaResult<&syn::DataStruct> {
    let syn::Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(input, "expected a struct").into());
    };
    Ok(data)
}

// Ensure the derive input is an enum and return the data.
pub fn as_derive_enum(input: &syn::DeriveInput) -> CodamaResult<&syn::DataEnum> {
    let syn::Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(input, "expected a enum").into());
    };
    Ok(data)
}

pub fn get_type_params(generics: &syn::Generics) -> proc_macro2::TokenStream {
    let type_params = generics
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
    let enum_type_params = if type_params.is_empty() {
        quote! {}
    } else {
        quote! { <#(#type_params),*> }
    };
    enum_type_params
}

pub fn lowercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + c.as_str(),
    }
}
