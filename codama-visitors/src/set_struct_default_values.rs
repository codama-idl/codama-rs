use codama_nodes::{CamelCaseString, Node, RegisteredTypeNode, ValueNode};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// Sets default values for named fields of structs selected by a
/// [`NodeSelector`] stack.
///
/// The Rust counterpart of `@codama/visitors`' `setStructDefaultValuesVisitor`,
/// built on [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer).
/// Each entry maps a stack (the struct's context) to `(fieldName, value)` pairs.
///
/// ```
/// use codama_nodes::{DefinedTypeNode, NumberTypeNode, NumberValueNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, U8};
/// use codama_visitors::{set_struct_default_values, TransformVisitor};
///
/// let data = StructTypeNode::new(vec![StructFieldTypeNode::new("bump", NumberTypeNode::le(U8))]);
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.defined_types.push(DefinedTypeNode::new("config", data));
/// let root = RootNode::new(program);
///
/// let mut visitor = set_struct_default_values([(
///     "[definedTypeNode]config",
///     vec![("bump".to_string(), NumberValueNode::new(0u8).into())],
/// )]);
/// let _root = visitor.visit_root(root);
/// ```
///
/// Note: this covers the struct path of the upstream visitor. Setting defaults
/// on instruction *arguments* (the second selector upstream applies) is a
/// follow-up.
pub fn set_struct_default_values<S: Into<String>>(
    map: impl IntoIterator<Item = (S, Vec<(String, ValueNode)>)>,
) -> BottomUpTransformer {
    let rules = map
        .into_iter()
        .map(|(stack, defaults)| {
            TransformRule::new(
                format!("{}.[structTypeNode]", stack.into()),
                move |node, _path| {
                    let Node::Type(RegisteredTypeNode::Struct(mut node)) = node else {
                        return node;
                    };
                    for field in node.fields.iter_mut() {
                        if let Some((_, value)) = defaults
                            .iter()
                            .find(|(name, _)| CamelCaseString::new(name) == field.name)
                        {
                            field.default_value = Box::new(Some(value.clone()));
                        }
                    }
                    Node::Type(RegisteredTypeNode::Struct(node))
                },
            )
        })
        .collect();
    bottom_up_transformer(rules)
}
