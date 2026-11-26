//! Assertions for the tests of the directive parsers.

macro_rules! assert_type {
    ({$($attr:tt)*}, $expected:expr) => {
        {
            let meta: codama_syn_helpers::Meta = syn::parse_quote! { type = $($attr)* };
            let node = $crate::TypeDirective::parse(&meta)
                .unwrap()
                .node
                .map(codama_nodes::Node::from);
            assert_eq!(node, $crate::Resolvable::Resolved($expected));
        }
    };
}

macro_rules! assert_type_err {
    ({$($attr:tt)*}, $expected:expr) => {
        {
            let meta: codama_syn_helpers::Meta = syn::parse_quote! { type = $($attr)* };
            let message = $crate::TypeDirective::parse(&meta).unwrap_err().to_string();
            assert_eq!(message, $expected);
        }
    };
}

macro_rules! assert_value {
    ({$($attr:tt)*}, $expected:expr) => {
        {
            let meta: codama_syn_helpers::Meta = syn::parse_quote! { default_value = $($attr)* };
            let node = $crate::DefaultValueDirective::parse(&meta).unwrap().node;
            assert_eq!(node, $crate::Resolvable::Resolved($expected));
        }
    };
}

macro_rules! assert_value_err {
    ({$($attr:tt)*}, $expected:expr) => {
        {
            let meta: codama_syn_helpers::Meta = syn::parse_quote! { default_value = $($attr)* };
            let message = $crate::DefaultValueDirective::parse(&meta).unwrap_err().to_string();
            assert_eq!(message, $expected);
        }
    };
}

pub(crate) use {assert_type, assert_type_err, assert_value, assert_value_err};
