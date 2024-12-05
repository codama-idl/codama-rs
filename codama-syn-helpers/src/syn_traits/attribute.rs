use codama_errors::CodamaResult;

pub trait Attribute {
    fn get_self(&self) -> &syn::Attribute;

    /// Ensure the attribute meta is a path.
    fn as_path(&self) -> CodamaResult<&syn::Path> {
        let this = self.get_self();
        match &this.meta {
            syn::Meta::Path(path) => Ok(path),
            _ => Err(
                syn::Error::new_spanned(this, "expected a Path attribute — e.g. `#[serde]`.")
                    .into(),
            ),
        }
    }

    /// Ensure the attribute meta is a path.
    fn as_list(&self) -> CodamaResult<&syn::MetaList> {
        let this = self.get_self();
        match &this.meta {
            syn::Meta::List(list) => Ok(list),
            _ => Err(syn::Error::new_spanned(
                this,
                "expected a MetaList attribute — e.g. `#[derive(Debug, PartialEq)]`.",
            )
            .into()),
        }
    }

    /// Ensure the attribute meta is a path.
    fn as_name_value(&self) -> CodamaResult<&syn::MetaNameValue> {
        let this = self.get_self();
        match &this.meta {
            syn::Meta::NameValue(name_value) => Ok(name_value),
            _ => Err(syn::Error::new_spanned(
                this,
                "expected a MetaNameValue attribute — e.g. `#[foo = 42]`.",
            )
            .into()),
        }
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
