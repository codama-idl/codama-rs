use codama_errors::CodamaResult;

pub trait Attribute {
    fn get_self(&self) -> &syn::Attribute;

    /// Ensure the attribute meta is a path.
    fn as_path(&self) -> CodamaResult<&syn::Path> {
        self.get_self().meta.require_path_only().map_err(Into::into)
    }

    /// Ensure the attribute meta is a path.
    fn as_list(&self) -> CodamaResult<&syn::MetaList> {
        self.get_self().meta.require_list().map_err(Into::into)
    }

    /// Ensure the attribute meta is a path.
    fn as_name_value(&self) -> CodamaResult<&syn::MetaNameValue> {
        self.get_self()
            .meta
            .require_name_value()
            .map_err(Into::into)
    }
}

impl Attribute for syn::Attribute {
    fn get_self(&self) -> &syn::Attribute {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_build;
    use quote::quote;

    #[test]
    fn as_path_ok() {
        let attribute = syn_build::attribute(quote! { #[serde] });
        assert!(matches!(attribute.as_path(), Ok(_)));
    }

    #[test]
    fn as_path_err() {
        let attribute = syn_build::attribute(quote! { #[derive(Debug)] });
        assert!(matches!(attribute.as_path(), Err(_)));
    }

    #[test]
    fn as_list_ok() {
        let attribute = syn_build::attribute(quote! { #[derive(Debug)] });
        assert!(matches!(attribute.as_list(), Ok(_)));
    }

    #[test]
    fn as_list_err() {
        let attribute = syn_build::attribute(quote! { #[serde] });
        assert!(matches!(attribute.as_list(), Err(_)));
    }

    #[test]
    fn as_name_value_ok() {
        let attribute = syn_build::attribute(quote! { #[foo = 42] });
        assert!(matches!(attribute.as_name_value(), Ok(_)));
    }

    #[test]
    fn as_name_value_err() {
        let attribute = syn_build::attribute(quote! { #[serde] });
        assert!(matches!(attribute.as_name_value(), Err(_)));
    }
}
