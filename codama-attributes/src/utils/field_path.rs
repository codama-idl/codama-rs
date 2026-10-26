use codama_nodes::{CamelCaseString, Path};
use codama_syn_helpers::extensions::*;
use quote::ToTokens;

/// Builds a path from a field name or a path such as `header.kind` or `fees[0]`.
/// Each identifier segment is camelCased, so the path matches the camelCased field identifiers.
/// The error names the input as written, since camelCasing can change it (`1st` gives `1St`).
pub fn field_path<T: ToTokens>(name: &str, tokens: T) -> syn::Result<Path> {
    let path = name
        .split('.')
        .map(|segment| {
            let (head, indices) = segment.split_at(segment.find('[').unwrap_or(segment.len()));
            format!("{}{indices}", CamelCaseString::new(head).as_str())
        })
        .collect::<Vec<_>>()
        .join(".");
    Path::try_from(path).map_err(|_| tokens.error(format!("Invalid path: {name}")))
}
