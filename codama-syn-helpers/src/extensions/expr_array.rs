use crate::PathList;
use quote::ToTokens;
use syn::ExprArray;

pub trait ExprArrayExtension {
    fn get_self(&self) -> &ExprArray;

    /// Converts the expression array into a PathList with a dummy path.
    fn as_path_list(&self) -> PathList {
        let this = self.get_self();
        PathList {
            path: syn::parse_quote!(implicit_array),
            eq_token: None,
            delimiter: syn::MacroDelimiter::Bracket(this.bracket_token),
            tokens: this.elems.to_token_stream(),
        }
    }
}

impl ExprArrayExtension for ExprArray {
    fn get_self(&self) -> &ExprArray {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{extensions::*, Meta};

    #[test]
    fn as_path_list_empty() {
        let expr: ExprArray = syn::parse_quote! { [] };
        let pl = expr.as_path_list();
        let metas = pl.parse_metas().unwrap();
        assert_eq!(pl.path.to_string(), "implicit_array");
        assert_eq!(metas.len(), 0);
    }

    #[test]
    fn as_path_list() {
        let expr: ExprArray = syn::parse_quote! { [x = 1, y = 2, options(42)] };
        let metas = expr.as_path_list().parse_metas().unwrap();
        assert_eq!(metas.len(), 3);
        assert!(matches!(&metas[0], Meta::PathValue(pv) if pv.path.is_ident("x")));
        assert!(matches!(&metas[1], Meta::PathValue(pv) if pv.path.is_ident("y")));
        assert!(matches!(&metas[2], Meta::PathList(pv) if pv.path.is_ident("options")));
    }
}
