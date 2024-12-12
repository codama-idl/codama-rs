use syn::Item;

pub trait ItemExtension {
    fn get_self(&self) -> &Item;

    fn attributes(&self) -> Vec<&syn::Attribute> {
        let attrs = match self.get_self() {
            Item::Const(i) => Some(&i.attrs),
            Item::Enum(i) => Some(&i.attrs),
            Item::ExternCrate(i) => Some(&i.attrs),
            Item::Fn(i) => Some(&i.attrs),
            Item::ForeignMod(i) => Some(&i.attrs),
            Item::Impl(i) => Some(&i.attrs),
            Item::Macro(i) => Some(&i.attrs),
            Item::Mod(i) => Some(&i.attrs),
            Item::Static(i) => Some(&i.attrs),
            Item::Struct(i) => Some(&i.attrs),
            Item::Trait(i) => Some(&i.attrs),
            Item::TraitAlias(i) => Some(&i.attrs),
            Item::Type(i) => Some(&i.attrs),
            Item::Union(i) => Some(&i.attrs),
            Item::Use(i) => Some(&i.attrs),
            _ => None,
        };

        match attrs {
            Some(attrs) => attrs.iter().collect(),
            None => vec![],
        }
    }
}

impl ItemExtension for Item {
    fn get_self(&self) -> &Item {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributes() {
        let r#struct: Item = syn::parse_quote! {
            #[derive(Debug)]
            struct Foo(u32);
        };
        assert!(matches!(
            r#struct.attributes().as_slice(),
            [syn::Attribute { .. }]
        ));
    }
}
