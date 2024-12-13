use super::ToTokensExtension;
use codama_errors::CodamaResult;
use syn::Fields;

pub trait FieldsExtension {
    fn get_self(&self) -> &Fields;

    fn single_unnamed_field(&self) -> CodamaResult<&syn::Field> {
        let this = self.get_self();
        match this {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Ok(&fields.unnamed[0]),
            _ => Err(this
                .error("expected a single unnamed field in the variant")
                .into()),
        }
    }
}

impl FieldsExtension for Fields {
    fn get_self(&self) -> &Fields {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_unnamed_field_ok() {
        let r#struct: syn::ItemStruct = syn::parse_quote! { struct Foo(u32); };
        assert!(r#struct.fields.single_unnamed_field().is_ok());
    }

    #[test]
    fn single_unnamed_field_err() {
        let r#struct: syn::ItemStruct = syn::parse_quote! { struct Foo(u32, u64); };
        assert!(r#struct.fields.single_unnamed_field().is_err());
    }
}
