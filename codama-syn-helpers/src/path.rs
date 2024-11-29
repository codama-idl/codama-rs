use crate::GenericArgumentsHelper;

pub struct PathHelper<'a>(pub &'a syn::Path);

impl PathHelper<'_> {
    /// Returns all segment idents joined by "::" except the last one.
    /// E.g. for `a::b<B>::c::Option<T>` it returns `a::b::c`.
    pub fn prefix(&self) -> String {
        self.0
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()[..self.0.segments.len() - 1]
            .join("::")
    }

    /// Returns the last segment.
    pub fn last(&self) -> &syn::PathSegment {
        self.0.segments.last().unwrap()
    }

    /// Returns the ident of the last segment as a string.
    pub fn last_indent(&self) -> String {
        self.last().ident.to_string()
    }

    /// Returns the generic arguments of the last segment.
    /// E.g. for `a::b::c::Option<'a, T, U>` it returns `GenericArgumentsHelper(Some(['a, T, U]))`.
    /// E.g. for `a::b::c::u32` it returns `GenericArgumentsHelper(None)`.
    pub fn generic_arguments(&self) -> GenericArgumentsHelper {
        match &self.last().arguments {
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => GenericArgumentsHelper(Some(args)),
            _ => GenericArgumentsHelper(None),
        }
    }
}
