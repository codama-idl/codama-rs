use crate::Meta;
use syn::punctuated::Punctuated;

pub trait MetaList {
    fn get_self(&self) -> &syn::MetaList;

    /// Parse all metas in the list.
    fn parse_metas(&self, logic: impl FnMut(Meta) -> syn::Result<()>) -> syn::Result<()> {
        self.parse_comma_args::<Meta>()?
            .into_iter()
            .try_for_each(logic)
            .map_err(Into::into)
    }

    /// Parse all arguments as comma-separated types.
    fn parse_comma_args<T: syn::parse::Parse>(&self) -> syn::Result<Vec<T>> {
        self.get_self()
            .parse_args_with(Punctuated::<T, syn::Token![,]>::parse_terminated)
            .map(|metas| metas.into_iter().collect::<Vec<_>>())
            .map_err(Into::into)
    }
}

impl MetaList for syn::MetaList {
    fn get_self(&self) -> &syn::MetaList {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syn_traits::{Expr, Path};

    #[test]
    fn parse_metas() {
        let list = syn::parse_str::<syn::MetaList>("foo(one, two, three = 42)").unwrap();
        let mut items = Vec::new();
        list.parse_metas(|meta| {
            items.push(meta);
            Ok(())
        })
        .unwrap();

        assert_eq!(items.len(), 3);
        assert!(items[0].as_path().unwrap().is_strict("one"));
        assert!(items[1].as_path().unwrap().is_strict("two"));
        let name_value = items[2].as_name_value().unwrap();
        assert!(name_value.path.is_strict("three"));
        assert_eq!(name_value.value.as_literal_integer::<usize>().unwrap(), 42);
    }
}
