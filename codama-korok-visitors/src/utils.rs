use codama_errors::CodamaResult;
use codama_nodes::StructFieldTypeNode;
use std::collections::HashSet;
use syn::spanned::Spanned;

/// Fails on the first field whose identifier an earlier field already has.
pub(crate) fn assert_unique_field_identifiers<'a>(
    fields: impl IntoIterator<Item = &'a StructFieldTypeNode>,
    parent: &str,
    ast: &impl Spanned,
) -> CodamaResult<()> {
    let mut identifiers = HashSet::new();
    let duplicate = fields
        .into_iter()
        .find(|field| !identifiers.insert(&field.identifier));
    match duplicate {
        Some(field) => {
            let message = format!(
                "Field `{}` is not unique in {}",
                field.identifier.as_str(),
                parent
            );
            Err(syn::Error::new(ast.span(), message).into())
        }
        None => Ok(()),
    }
}
