use crate::identify_field_types_visitor::utils::get_node_from_type;
use codama_nodes::{
    HasTransforms, IntegerTypeNode, Node, SizePrefixTransformNode, StringTypeNode, U32,
};
use quote::quote;

#[test]
fn it_identifies_string_types() {
    let expected: Option<Node> = Some(
        StringTypeNode::utf8()
            .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32)))
            .into(),
    );
    assert_eq!(get_node_from_type(quote! { String }), expected);
    assert_eq!(get_node_from_type(quote! { std::string::String }), expected);
    assert_eq!(get_node_from_type(quote! { String<T> }), None);
}
