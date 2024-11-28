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

pub struct GenericArgumentsHelper<'a>(
    pub Option<&'a syn::punctuated::Punctuated<syn::GenericArgument, syn::Token![,]>>,
);

impl GenericArgumentsHelper<'_> {
    /// Filters out all generic arguments that are not types.
    /// E.g. for `Option<'a, T, U>` it returns `[T, U]`.
    pub fn types(&self) -> Vec<&syn::Type> {
        match self.0 {
            Some(args) => args
                .iter()
                .filter_map(|arg| match arg {
                    syn::GenericArgument::Type(ty) => Some(ty),
                    _ => None,
                })
                .collect(),
            None => vec![],
        }
    }

    /// Returns the first genertic type argument if there is one.
    /// E.g. for `Vec<'a, T, U>` it returns `Some(T)`.
    pub fn first_type(&self) -> Option<&syn::Type> {
        self.types().first().copied()
    }
}

pub struct ExprHelper<'a>(pub &'a syn::Expr);

impl ExprHelper<'_> {
    /// Returns the integer value of the expression if it is a literal integer.
    pub fn as_literal_integer<T>(&self) -> Option<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        match self.0 {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(value),
                ..
            }) => value.base10_parse::<T>().ok(),
            _ => None,
        }
    }
}
