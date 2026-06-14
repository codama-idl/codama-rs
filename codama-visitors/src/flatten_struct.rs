use codama_nodes::{
    CamelCaseString, Node, RegisteredTypeNode, StructFieldTypeNode, StructTypeNode, TypeNode,
};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};
use std::collections::HashSet;

/// Which fields of a struct to inline.
#[derive(Debug, Clone)]
pub enum FlattenStruct {
    /// Inline every field whose type is itself a struct.
    All,
    /// Inline only the named fields (when their type is a struct).
    Fields(Vec<String>),
}

/// Inlines nested struct fields into their parent struct, for structs selected
/// by a [`NodeSelector`] stack.
///
/// The Rust counterpart of `@codama/visitors`' `flattenStructVisitor`, built on
/// [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer). If
/// inlining would create duplicate field names, the struct is left unchanged
/// (upstream throws).
pub fn flatten_struct<S: Into<String>>(
    map: impl IntoIterator<Item = (S, FlattenStruct)>,
) -> BottomUpTransformer {
    let rules = map
        .into_iter()
        .map(|(stack, options)| {
            TransformRule::new(
                format!("{}.[structTypeNode]", stack.into()),
                move |node, _path| match node {
                    Node::Type(RegisteredTypeNode::Struct(node)) => Node::Type(
                        RegisteredTypeNode::Struct(flatten_struct_node(node, &options)),
                    ),
                    other => other,
                },
            )
        })
        .collect();
    bottom_up_transformer(rules)
}

/// Inlines the (selected) nested struct fields of `node`. Returns `node`
/// unchanged if inlining would produce duplicate field names.
pub fn flatten_struct_node(node: StructTypeNode, options: &FlattenStruct) -> StructTypeNode {
    let should_inline = |name: &CamelCaseString| match options {
        FlattenStruct::All => true,
        FlattenStruct::Fields(fields) => fields.iter().any(|f| &CamelCaseString::new(f) == name),
    };

    let original = node.clone();
    let mut inlined: Vec<StructFieldTypeNode> = Vec::with_capacity(node.fields.len());
    for field in node.fields {
        if should_inline(&field.name) && matches!(&*field.r#type, TypeNode::Struct(_)) {
            if let TypeNode::Struct(inner) = *field.r#type {
                inlined.extend(inner.fields);
            }
        } else {
            inlined.push(field);
        }
    }

    if has_duplicate_names(&inlined) {
        original
    } else {
        StructTypeNode::new(inlined)
    }
}

fn has_duplicate_names(fields: &[StructFieldTypeNode]) -> bool {
    let mut seen = HashSet::with_capacity(fields.len());
    !fields.iter().all(|f| seen.insert(&f.name))
}
