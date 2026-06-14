use crate::rename_helpers::{rename_enum_node, rename_map, rename_struct_node, scoped_selector};
use codama_nodes::{DefinedTypeLinkNode, Docs, LinkNode, Node, TypeNode};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// A partial update applied to a matched `definedTypeNode`; only the `Some`
/// fields are changed. `rename` entries rename inner struct fields / enum
/// variants; `delete` removes the defined type entirely.
#[derive(Debug, Clone, Default)]
pub struct DefinedTypeUpdate {
    name: Option<String>,
    docs: Option<Docs>,
    data: Vec<(String, String)>,
    delete: bool,
}

impl DefinedTypeUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn docs(mut self, docs: impl Into<Docs>) -> Self {
        self.docs = Some(docs.into());
        self
    }

    /// Rename an inner struct field or enum variant (`from` -> `to`).
    pub fn rename(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.data.push((from.into(), to.into()));
        self
    }

    /// Mark the defined type for deletion.
    pub fn delete(mut self) -> Self {
        self.delete = true;
        self
    }
}

/// Updates defined types selected by name: metadata, inner field/variant renames
/// (via the `data` map), and deletion. On rename, every `definedTypeLinkNode`
/// pointing at the old name is rewritten to the new one.
///
/// The Rust counterpart of `@codama/visitors`' `updateDefinedTypesVisitor`, built
/// on [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer).
///
/// ```
/// use codama_nodes::{DefinedTypeNode, NumberTypeNode, ProgramNode, RootNode, U8};
/// use codama_visitors::{update_defined_types, DefinedTypeUpdate, TransformVisitor};
///
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.defined_types.push(DefinedTypeNode::new("oldType", NumberTypeNode::le(U8)));
/// let root = RootNode::new(program);
///
/// let root = update_defined_types([("oldType", DefinedTypeUpdate::new().name("newType"))]).visit_root(root);
/// assert_eq!(root.program.defined_types[0].name.as_ref(), "newType");
/// ```
pub fn update_defined_types<S: Into<String>>(
    map: impl IntoIterator<Item = (S, DefinedTypeUpdate)>,
) -> BottomUpTransformer {
    let mut rules = Vec::new();
    let mut deletes: Vec<String> = Vec::new();
    for (name, update) in map {
        let name = name.into();

        if update.delete {
            deletes.push(scoped_selector("definedTypeNode", &name));
            continue;
        }

        let new_name = update.name.clone();
        let renames = rename_map(update.data.clone());

        rules.push(TransformRule::new(
            scoped_selector("definedTypeNode", &name),
            move |node, _path| {
                let Node::DefinedType(mut defined_type) = node else {
                    return node;
                };
                if let Some(name) = &update.name {
                    defined_type.name = name.clone().into();
                }
                if let Some(docs) = &update.docs {
                    defined_type.docs = docs.clone();
                }
                if !renames.is_empty() {
                    let new_type = match (*defined_type.r#type).clone() {
                        TypeNode::Struct(node) => rename_struct_node(node, &renames).into(),
                        TypeNode::Enum(node) => rename_enum_node(node, &renames).into(),
                        other => other,
                    };
                    defined_type.r#type = Box::new(new_type);
                }
                Node::DefinedType(defined_type)
            },
        ));

        if let Some(new_name) = new_name {
            rules.push(TransformRule::new(
                scoped_selector("definedTypeLinkNode", &name),
                move |node, _path| match node {
                    Node::Link(LinkNode::DefinedType(link)) => {
                        Node::Link(LinkNode::DefinedType(DefinedTypeLinkNode {
                            name: new_name.clone().into(),
                            program: link.program, // keep any cross-program reference
                        }))
                    }
                    other => other,
                },
            ));
        }
    }
    bottom_up_transformer(rules).also_delete(deletes)
}
