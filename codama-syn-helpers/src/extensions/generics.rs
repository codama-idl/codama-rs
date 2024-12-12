use proc_macro2::TokenStream;
use quote::quote;
use syn::Generics;

pub trait GenericsExtension {
    fn get_self(&self) -> &Generics;

    fn param_idents(&self) -> TokenStream {
        let this = self.get_self();
        let idents = this
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
        match idents.is_empty() {
            true => quote! {},
            false => quote! { <#(#idents),*> },
        }
    }

    fn block_wrappers(&self) -> (TokenStream, TokenStream) {
        let this = self.get_self();
        let declarations = &this.params;
        let declarations = match &this.params.is_empty() {
            true => quote! {},
            false => quote! { <#declarations> },
        };
        let usages = self.param_idents();
        let where_clause = &this.where_clause;
        (quote! { #declarations }, quote! { #usages #where_clause })
    }
}

impl GenericsExtension for Generics {
    fn get_self(&self) -> &Generics {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn param_idents() {
        let r#struct: syn::ItemStruct = syn::parse_quote! { struct Foo(u32); };
        assert_eq!(
            r#struct.generics.param_idents().to_string(),
            quote! {}.to_string()
        );

        let r#struct: syn::ItemStruct =
            syn::parse_quote! { struct Foo<'a, T: Clone, U: PartialEq> where U: Eq (T); };
        assert_eq!(
            r#struct.generics.param_idents().to_string(),
            quote! { <'a, T, U> }.to_string()
        );
    }

    #[test]
    fn block_wrappers() {
        let r#struct: syn::ItemStruct = syn::parse_quote! { struct Foo(u32); };
        let (pre, post) = r#struct.generics.block_wrappers();
        assert_eq!(
            quote! { impl #pre Bar for Foo #post {} }.to_string(),
            quote! { impl Bar for Foo {} }.to_string()
        );

        let r#struct: syn::ItemStruct =
            syn::parse_quote! { struct Foo<'a, T: Clone, U: PartialEq> where U: Eq (T); };
        let (pre, post) = r#struct.generics.block_wrappers();
        assert_eq!(
            quote! { impl #pre Bar for Foo #post {} }.to_string(),
            quote! { impl<'a, T: Clone, U: PartialEq> Bar for Foo<'a, T, U> where U: Eq (T) {} }
                .to_string()
        );
    }
}
