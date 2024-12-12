#[macro_export]
macro_rules! assert_node {
    ({$($attr:tt)*}, $expected:expr) => {
        {
            let meta: codama_syn_helpers::Meta = syn::parse_quote! { node($($attr)*) };
            let node = crate::NodeDirective::try_from(&meta).unwrap().node;
            assert_eq!(node, $expected);
        }
    };
}

#[macro_export]
macro_rules! assert_node_err {
    ({$($attr:tt)*}, $expected:expr) => {
        {
            let meta: codama_syn_helpers::Meta = syn::parse_quote! { node($($attr)*) };
            let message = crate::NodeDirective::try_from(&meta).unwrap_err().to_string();
            assert!(message.contains($expected), "Expected error containing '{}', but got '{}'", $expected, message);
        }
    };
}
