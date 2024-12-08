#[macro_export]
macro_rules! assert_node {
    (#[node($($attr:tt)*)], $expected:expr) => {
        {
            let ast = syn_build::attribute(quote! { #[node($($attr)*)] });
            let node = NodeAttribute::parse(&ast).unwrap().node;
            assert_eq!(node, $expected);
        }
    };
}

#[macro_export]
macro_rules! assert_node_err {
    (#[node($($attr:tt)*)], $expected:expr) => {
        {
            let ast = syn_build::attribute(quote! { #[node($($attr)*)] });
            let message = NodeAttribute::parse(&ast).unwrap_err().to_string();
            assert!(message.contains($expected), "Expected error containing '{}', but got '{}'", $expected, message);
        }
    };
}
