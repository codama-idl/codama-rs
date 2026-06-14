use codama_nodes::{DefinedTypeLinkNode, Docs, LinkNode, Node};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// A partial update applied to a matched `definedTypeNode`; only the `Some`
/// fields are changed.
#[derive(Debug, Clone, Default)]
pub struct DefinedTypeUpdate {
    name: Option<String>,
    docs: Option<Docs>,
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
}

/// Updates defined types selected by name. On rename, every `definedTypeLinkNode`
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
///
/// Note: renaming the type's *inner* fields (the upstream `data` map applied via
/// `renameStructNode`/`renameEnumNode`) is a follow-up.
pub fn update_defined_types<S: Into<String>>(
    map: impl IntoIterator<Item = (S, DefinedTypeUpdate)>,
) -> BottomUpTransformer {
    let mut rules = Vec::new();
    for (name, update) in map {
        let name = name.into();
        let new_name = update.name.clone();

        rules.push(TransformRule::new(
            format!("[definedTypeNode]{name}"),
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
                Node::DefinedType(defined_type)
            },
        ));

        if let Some(new_name) = new_name {
            rules.push(TransformRule::new(
                format!("[definedTypeLinkNode]{name}"),
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
    bottom_up_transformer(rules)
}
