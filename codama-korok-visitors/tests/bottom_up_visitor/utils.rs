use quote::quote;

pub fn get_mock_syn_type() -> syn::Type {
    syn::Type::Verbatim(quote! {})
}

pub fn get_mock_syn_field(ident: Option<syn::Ident>) -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        mutability: syn::FieldMutability::None,
        ident,
        colon_token: None,
        ty: get_mock_syn_type(),
    }
}
