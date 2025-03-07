#[macro_export]
macro_rules! assert_type {
    ({$($attr:tt)*}, $expected:expr) => {
        {
            let meta: codama_syn_helpers::Meta = syn::parse_quote! { type = $($attr)* };
            let node = $crate::TypeDirective::parse(&meta).unwrap().node;
            assert_eq!(node, $expected);
        }
    };
}

#[macro_export]
macro_rules! assert_type_err {
    ({$($attr:tt)*}, $expected:expr) => {
        {
            let meta: codama_syn_helpers::Meta = syn::parse_quote! { type = $($attr)* };
            let message = $crate::TypeDirective::parse(&meta).unwrap_err().to_string();
            assert_eq!(message, $expected);
        }
    };
}

#[macro_export]
macro_rules! assert_value {
    ({$($attr:tt)*}, $expected:expr) => {
        {
            let meta: codama_syn_helpers::Meta = syn::parse_quote! { default_value = $($attr)* };
            let node = $crate::DefaultValueDirective::parse(&meta).unwrap().node;
            assert_eq!(node, $expected);
        }
    };
}

#[macro_export]
macro_rules! assert_value_err {
    ({$($attr:tt)*}, $expected:expr) => {
        {
            let meta: codama_syn_helpers::Meta = syn::parse_quote! { default_value = $($attr)* };
            let message = $crate::DefaultValueDirective::parse(&meta).unwrap_err().to_string();
            assert_eq!(message, $expected);
        }
    };
}
