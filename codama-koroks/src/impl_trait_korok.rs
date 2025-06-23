use codama_errors::CodamaResult;

#[derive(Debug, PartialEq)]
pub struct ImplTraitKorok {
    pub path: String,
}

impl ImplTraitKorok {
    pub fn parse(
        trait_: &Option<(Option<syn::Token![!]>, syn::Path, syn::Token![for])>,
    ) -> CodamaResult<Option<Self>> {
        Ok(trait_.as_ref().map(|(_, path, _)| ImplTraitKorok {
            path: path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
        }))
    }
}
