pub trait Item {
    fn get_self(&self) -> &syn::Item;

    fn attributes(&self) -> Vec<&syn::Attribute> {
        let attrs = match self.get_self() {
            syn::Item::Const(i) => Some(&i.attrs),
            syn::Item::Enum(i) => Some(&i.attrs),
            syn::Item::ExternCrate(i) => Some(&i.attrs),
            syn::Item::Fn(i) => Some(&i.attrs),
            syn::Item::ForeignMod(i) => Some(&i.attrs),
            syn::Item::Impl(i) => Some(&i.attrs),
            syn::Item::Macro(i) => Some(&i.attrs),
            syn::Item::Mod(i) => Some(&i.attrs),
            syn::Item::Static(i) => Some(&i.attrs),
            syn::Item::Struct(i) => Some(&i.attrs),
            syn::Item::Trait(i) => Some(&i.attrs),
            syn::Item::TraitAlias(i) => Some(&i.attrs),
            syn::Item::Type(i) => Some(&i.attrs),
            syn::Item::Union(i) => Some(&i.attrs),
            syn::Item::Use(i) => Some(&i.attrs),
            _ => None,
        };

        match attrs {
            Some(attrs) => attrs.iter().collect(),
            None => vec![],
        }
    }
}

impl Item for syn::Item {
    fn get_self(&self) -> &syn::Item {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributes() {
        let r#struct: syn::Item = syn::parse_quote! {
            #[derive(Debug)]
            struct Foo(u32);
        };
        assert!(matches!(
            r#struct.attributes().as_slice(),
            [syn::Attribute { .. }]
        ));
    }
}
