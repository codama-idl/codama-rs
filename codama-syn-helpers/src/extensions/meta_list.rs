use crate::Meta;
use syn::{punctuated::Punctuated, MetaList};

pub trait MetaListExtension {
    fn get_self(&self) -> &MetaList;

    /// Iterate over all metas in the list.
    fn each(&self, logic: impl FnMut(Meta) -> syn::Result<()>) -> syn::Result<()> {
        self.parse_metas()?.into_iter().try_for_each(logic)
    }

    /// Parse all metas in the list.
    fn parse_metas(&self) -> syn::Result<Vec<Meta>> {
        self.parse_comma_args::<Meta>()
    }

    /// Parse all arguments as comma-separated types.
    fn parse_comma_args<T: syn::parse::Parse>(&self) -> syn::Result<Vec<T>> {
        self.get_self()
            .parse_args_with(Punctuated::<T, syn::Token![,]>::parse_terminated)
            .map(|metas| metas.into_iter().collect::<Vec<_>>())
    }
}

impl MetaListExtension for MetaList {
    fn get_self(&self) -> &MetaList {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extensions::*;

    #[test]
    fn each() {
        let list = syn::parse_str::<MetaList>("foo(one, two, three = 42)").unwrap();
        let mut items = Vec::new();
        list.each(|meta| {
            items.push(meta);
            Ok(())
        })
        .unwrap();

        assert_eq!(items.len(), 3);
        assert!(items[0].as_path().unwrap().is_strict("one"));
        assert!(items[1].as_path().unwrap().is_strict("two"));
        let meta = items[2].as_path_value().unwrap();
        let expr = meta.value.as_expr().unwrap();
        assert!(meta.path.is_strict("three"));
        assert_eq!(expr.as_unsigned_integer::<usize>().unwrap(), 42);
    }
}
