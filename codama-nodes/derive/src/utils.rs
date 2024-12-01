use codama_errors::CodamaResult;
use codama_syn_helpers::syn_traits::*;

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
    generics.param_idents()
}

pub fn lowercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + c.as_str(),
    }
}
