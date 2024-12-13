use super::ToTokensExtension;
use codama_errors::CodamaResult;
use syn::DeriveInput;

pub trait DeriveInputExtension {
    fn get_self(&self) -> &DeriveInput;

    /// Ensure the derive input is a struct and return the data.
    fn as_struct(&self) -> CodamaResult<&syn::DataStruct> {
        let this = self.get_self();
        match &this.data {
            syn::Data::Struct(data) => Ok(data),
            _ => Err(this.error("expected a struct").into()),
        }
    }

    /// Ensure the derive input is an enum and return the data.
    fn as_enum(&self) -> CodamaResult<&syn::DataEnum> {
        let this = self.get_self();
        match &this.data {
            syn::Data::Enum(data) => Ok(data),
            _ => Err(this.error("expected an enum").into()),
        }
    }
}

impl DeriveInputExtension for DeriveInput {
    fn get_self(&self) -> &DeriveInput {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_struct_ok() {
        let derive_input: DeriveInput = syn::parse_quote! { struct Foo(u32); };
        assert!(derive_input.as_struct().is_ok());
    }

    #[test]
    fn as_struct_err() {
        let derive_input: DeriveInput = syn::parse_quote! { enum Foo { Bar } };
        assert!(derive_input.as_struct().is_err());
    }

    #[test]
    fn as_enum_ok() {
        let derive_input: DeriveInput = syn::parse_quote! { enum Foo { Bar } };
        assert!(derive_input.as_enum().is_ok());
    }

    #[test]
    fn as_enum_err() {
        let derive_input: DeriveInput = syn::parse_quote! { struct Foo(u32); };
        assert!(derive_input.as_enum().is_err());
    }
}
