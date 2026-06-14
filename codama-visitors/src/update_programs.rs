use codama_nodes::{Docs, LinkNode, Node, ProgramLinkNode, ProgramNode};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// A partial update applied to a matched [`ProgramNode`]; only the `Some` fields
/// are changed. (Child collections — accounts, instructions, etc. — are updated
/// by their own visitors, not here.)
#[derive(Debug, Clone, Default)]
pub struct ProgramUpdate {
    name: Option<String>,
    public_key: Option<String>,
    version: Option<String>,
    docs: Option<Docs>,
}

impl ProgramUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn public_key(mut self, public_key: impl Into<String>) -> Self {
        self.public_key = Some(public_key.into());
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn docs(mut self, docs: impl Into<Docs>) -> Self {
        self.docs = Some(docs.into());
        self
    }
}

/// Updates fields of programs selected by name. On rename, every
/// `programLinkNode` pointing at the old name is rewritten to the new one, so
/// cross-references stay valid.
///
/// The Rust counterpart of `@codama/visitors`' `updateProgramsVisitor`, built on
/// [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer).
///
/// ```
/// use codama_nodes::{ProgramNode, RootNode};
/// use codama_visitors::{update_programs, ProgramUpdate, TransformVisitor};
///
/// let program = ProgramNode::new("oldName", "Myprogram1111111111111111111111111111111111");
/// let root = RootNode::new(program);
///
/// let root = update_programs([("oldName", ProgramUpdate::new().name("newName"))]).visit_root(root);
/// assert_eq!(root.program.name.as_ref(), "newName");
/// ```
pub fn update_programs<S: Into<String>>(
    map: impl IntoIterator<Item = (S, ProgramUpdate)>,
) -> BottomUpTransformer {
    let mut rules = Vec::new();
    for (name, update) in map {
        let name = name.into();
        let new_name = update.name.clone();

        rules.push(TransformRule::new(
            format!("[programNode]{name}"),
            move |node, _path| {
                let Node::Program(mut program) = node else {
                    return node;
                };
                apply_update(&mut program, &update);
                Node::Program(program)
            },
        ));

        if let Some(new_name) = new_name {
            rules.push(TransformRule::new(
                format!("[programLinkNode]{name}"),
                move |node, _path| match node {
                    Node::Link(LinkNode::Program(_)) => {
                        Node::Link(LinkNode::Program(ProgramLinkNode {
                            name: new_name.clone().into(),
                        }))
                    }
                    other => other,
                },
            ));
        }
    }
    bottom_up_transformer(rules)
}

fn apply_update(program: &mut ProgramNode, update: &ProgramUpdate) {
    if let Some(name) = &update.name {
        program.name = name.clone().into();
    }
    if let Some(public_key) = &update.public_key {
        program.public_key = public_key.clone();
    }
    if let Some(version) = &update.version {
        program.version = version.clone();
    }
    if let Some(docs) = &update.docs {
        program.docs = docs.clone();
    }
}
