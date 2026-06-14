use codama_nodes::{AccountLinkNode, Docs, LinkNode, Node, PdaLinkNode};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// A partial update applied to a matched `accountNode`; only the `Some` fields
/// are changed.
#[derive(Debug, Clone, Default)]
pub struct AccountUpdate {
    name: Option<String>,
    size: Option<u64>,
    docs: Option<Docs>,
}

impl AccountUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn docs(mut self, docs: impl Into<Docs>) -> Self {
        self.docs = Some(docs.into());
        self
    }
}

/// Updates accounts selected by name. On rename, references are kept valid: the
/// matching `accountLinkNode`s, the same-named `pdaNode`, and `pdaLinkNode`s are
/// all rewritten to the new name (mirroring the account/PDA naming convention).
///
/// The Rust counterpart of `@codama/visitors`' `updateAccountsVisitor`, built on
/// [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer).
///
/// ```
/// use codama_nodes::{AccountNode, ProgramNode, RootNode, StructTypeNode};
/// use codama_visitors::{update_accounts, AccountUpdate, TransformVisitor};
///
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.accounts.push(AccountNode::new("oldName", StructTypeNode::new(vec![])));
/// let root = RootNode::new(program);
///
/// let root = update_accounts([("oldName", AccountUpdate::new().name("newName").size(8))]).visit_root(root);
/// assert_eq!(root.program.accounts[0].name.as_ref(), "newName");
/// assert_eq!(root.program.accounts[0].size, Some(8));
/// ```
///
/// Note: deriving new PDAs from a `seeds` update (the upstream PDA upsert) and
/// renaming the account's inner data fields are follow-ups.
pub fn update_accounts<S: Into<String>>(
    map: impl IntoIterator<Item = (S, AccountUpdate)>,
) -> BottomUpTransformer {
    let mut rules = Vec::new();
    for (name, update) in map {
        let name = name.into();
        let new_name = update.name.clone();

        rules.push(TransformRule::new(
            format!("[accountNode]{name}"),
            move |node, _path| {
                let Node::Account(mut account) = node else {
                    return node;
                };
                if let Some(name) = &update.name {
                    account.name = name.clone().into();
                }
                if let Some(size) = update.size {
                    account.size = Some(size);
                }
                if let Some(docs) = &update.docs {
                    account.docs = docs.clone();
                }
                Node::Account(account)
            },
        ));

        let Some(new_name) = new_name else { continue };

        // Rewrite every reference to the old name so the IDL stays consistent.
        let account_link_name = new_name.clone();
        rules.push(TransformRule::new(
            format!("[accountLinkNode]{name}"),
            move |node, _path| match node {
                Node::Link(LinkNode::Account(link)) => {
                    Node::Link(LinkNode::Account(AccountLinkNode {
                        name: account_link_name.clone().into(),
                        program: link.program,
                    }))
                }
                other => other,
            },
        ));

        let pda_name = new_name.clone();
        rules.push(TransformRule::new(
            format!("[pdaNode]{name}"),
            move |node, _path| match node {
                Node::Pda(mut pda) => {
                    pda.name = pda_name.clone().into();
                    Node::Pda(pda)
                }
                other => other,
            },
        ));

        let pda_link_name = new_name;
        rules.push(TransformRule::new(
            format!("[pdaLinkNode]{name}"),
            move |node, _path| match node {
                Node::Link(LinkNode::Pda(link)) => Node::Link(LinkNode::Pda(PdaLinkNode {
                    name: pda_link_name.clone().into(),
                    program: link.program,
                })),
                other => other,
            },
        ));
    }
    bottom_up_transformer(rules)
}
