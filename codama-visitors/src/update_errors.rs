use codama_nodes::{Docs, Node};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// A partial update applied to a matched [`ErrorNode`]; only the `Some` fields
/// are changed.
#[derive(Debug, Clone, Default)]
pub struct ErrorUpdate {
    name: Option<String>,
    code: Option<u32>,
    message: Option<String>,
    docs: Option<Docs>,
}

impl ErrorUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn code(mut self, code: u32) -> Self {
        self.code = Some(code);
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn docs(mut self, docs: impl Into<Docs>) -> Self {
        self.docs = Some(docs.into());
        self
    }
}

/// Updates fields of error nodes selected by name.
///
/// The Rust counterpart of `@codama/visitors`' `updateErrorsVisitor`, built on
/// [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer). Each
/// entry maps an error name to the fields to change.
///
/// ```
/// use codama_nodes::{ErrorNode, ProgramNode, RootNode};
/// use codama_visitors::{update_errors, ErrorUpdate, TransformVisitor};
///
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.errors.push(ErrorNode { name: "oops".into(), code: 1, message: "Oops".into(), docs: Default::default() });
/// let root = RootNode::new(program);
///
/// let mut visitor = update_errors([("oops", ErrorUpdate::new().code(42))]);
/// let root = visitor.visit_root(root);
/// assert_eq!(root.program.errors[0].code, 42);
/// ```
///
/// Note: the upstream `{ delete: true }` variant is not supported yet, since the
/// transformer cannot drop nodes (see `BottomUpTransformer`).
pub fn update_errors<S: Into<String>>(
    map: impl IntoIterator<Item = (S, ErrorUpdate)>,
) -> BottomUpTransformer {
    let rules = map
        .into_iter()
        .map(|(name, update)| {
            TransformRule::new(format!("[errorNode]{}", name.into()), move |node, _path| {
                update_error(node, update.clone())
            })
        })
        .collect();
    bottom_up_transformer(rules)
}

fn update_error(node: Node, update: ErrorUpdate) -> Node {
    let Node::Error(mut error) = node else {
        return node;
    };
    if let Some(name) = update.name {
        error.name = name.into();
    }
    if let Some(code) = update.code {
        error.code = code;
    }
    if let Some(message) = update.message {
        error.message = message;
    }
    if let Some(docs) = update.docs {
        error.docs = docs;
    }
    Node::Error(error)
}
