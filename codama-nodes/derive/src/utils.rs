// Identify the inner type of a type — e.g. `Box<T>` -> `T`.
pub fn unwrap_inner_type<'a>(ty: &'a syn::Type, ident: &'a str) -> Option<&'a syn::Type> {
    // Get the path of the type. — e.g. `a::b::c::Option`.
    let syn::Type::Path(syn::TypePath { path, .. }) = ty else {
        return None;
    };

    // Only match single-segment paths whose ident is expected.
    if !is_single_path(path, ident) {
        return None;
    };
    let segment = &path.segments[0];

    // Get the generic arguments of the segment.
    let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        args: generic_args,
        ..
    }) = &segment.arguments
    else {
        return None;
    };

    // Only match types with a single generic argument.
    if generic_args.len() != 1 {
        return None;
    };

    // Ensure the generic argument is also a type.
    let syn::GenericArgument::Type(ty) = generic_args.first().unwrap() else {
        return None;
    };

    // Return the inner type.
    Some(ty)
}

pub fn is_single_path(path: &syn::Path, ident: &str) -> bool {
    path.segments.len() == 1 && path.segments[0].ident == ident
}

pub fn lowercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + c.as_str(),
    }
}
