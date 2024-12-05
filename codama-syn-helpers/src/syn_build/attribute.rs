use codama_errors::CodamaResult;
use proc_macro2::TokenStream;
use quote::quote;

/// E.g. `#[derive(Debug)]` or `#[foo = "42"]`
pub fn try_attribute(tt: TokenStream) -> CodamaResult<syn::Attribute> {
    let wrapper = quote! {
        #tt
        struct Foo {}
    };
    let ast = syn::parse2::<syn::ItemStruct>(wrapper)?;
    Ok(ast.attrs[0].clone())
}

/// E.g. `#[derive(Debug)]` or `#[foo = "42"]`
pub fn attribute(tt: TokenStream) -> syn::Attribute {
    try_attribute(tt).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_errors::CodamaError;

    #[test]
    fn path_attribute_ok() {
        let result = try_attribute(quote! { #[serde] });
        assert!(matches!(
            result,
            Ok(syn::Attribute {
                meta: syn::Meta::Path(_),
                ..
            })
        ));
    }

    #[test]
    fn list_attribute_ok() {
        let result = try_attribute(quote! { #[derive(Debug)] });
        assert!(matches!(
            result,
            Ok(syn::Attribute {
                meta: syn::Meta::List(_),
                ..
            })
        ));
    }

    #[test]
    fn name_value_attribute_ok() {
        let result = try_attribute(quote! { #[foo = 42] });
        assert!(matches!(
            result,
            Ok(syn::Attribute {
                meta: syn::Meta::NameValue(_),
                ..
            })
        ));
    }

    #[test]
    fn attribute_err() {
        let result = try_attribute(quote! { struct Foo {} });
        assert!(matches!(result, Err(CodamaError::Compilation(_))));
    }
}
