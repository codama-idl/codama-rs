use codama_errors::CodamaResult;

pub trait DeriveInput {
    fn get_self(&self) -> &syn::DeriveInput;

    /// Ensure the derive input is a struct and return the data.
    fn as_struct(&self) -> CodamaResult<&syn::DataStruct> {
        let this = self.get_self();
        match &this.data {
            syn::Data::Struct(data) => Ok(data),
            _ => Err(syn::Error::new_spanned(this, "expected a struct").into()),
        }
    }

    /// Ensure the derive input is an enum and return the data.
    fn as_enum(&self) -> CodamaResult<&syn::DataEnum> {
        let this = self.get_self();
        match &this.data {
            syn::Data::Enum(data) => Ok(data),
            _ => Err(syn::Error::new_spanned(this, "expected an enum").into()),
        }
    }
}

impl DeriveInput for syn::DeriveInput {
    fn get_self(&self) -> &syn::DeriveInput {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::quote;

    #[test]
    fn as_struct_ok() {
        let derive_input: syn::DeriveInput = syn_build::parse(quote! { struct Foo(u32); });
        assert!(matches!(derive_input.as_struct(), Ok(_)));
    }

    #[test]
    fn as_struct_err() {
        let derive_input: syn::DeriveInput = syn_build::parse(quote! { enum Foo { Bar } });
        assert!(matches!(derive_input.as_struct(), Err(_)));
    }

    #[test]
    fn as_enum_ok() {
        let derive_input: syn::DeriveInput = syn_build::parse(quote! { enum Foo { Bar } });
        assert!(matches!(derive_input.as_enum(), Ok(_)));
    }

    #[test]
    fn as_enum_err() {
        let derive_input: syn::DeriveInput = syn_build::parse(quote! { struct Foo(u32); });
        assert!(matches!(derive_input.as_enum(), Err(_)));
    }
}
