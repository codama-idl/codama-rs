use crate::rename_helpers::{rename_map, rename_struct_node, scoped_selector};
use codama_nodes::{
    AccountLinkNode, AccountNode, Docs, LinkNode, NestedTypeNodeTrait, Node, PdaLinkNode, PdaNode,
    PdaSeedNode, StructTypeNode,
};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};
use std::cell::RefCell;
use std::rc::Rc;

/// A partial update applied to a matched `accountNode`; only the `Some` fields
/// are changed. `rename` renames inner data fields; `seeds`/`pda` drive PDA
/// upserting; `delete` removes the account.
#[derive(Debug, Clone, Default)]
pub struct AccountUpdate {
    name: Option<String>,
    size: Option<u64>,
    docs: Option<Docs>,
    data: Vec<(String, String)>,
    seeds: Option<Vec<PdaSeedNode>>,
    pda: Option<PdaLinkNode>,
    delete: bool,
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

    /// Rename an inner data field (`from` -> `to`).
    pub fn rename(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.data.push((from.into(), to.into()));
        self
    }

    /// Provide the PDA seeds to derive (and upsert) a PDA for this account.
    pub fn seeds(mut self, seeds: Vec<PdaSeedNode>) -> Self {
        self.seeds = Some(seeds);
        self
    }

    /// Point the account at an explicit PDA link.
    pub fn pda(mut self, pda: PdaLinkNode) -> Self {
        self.pda = Some(pda);
        self
    }

    /// Mark the account for deletion.
    pub fn delete(mut self) -> Self {
        self.delete = true;
        self
    }
}

/// Updates accounts selected by name: metadata, inner data-field renames, PDA
/// derivation (a `seeds` update upserts a matching `pdaNode` on the program and
/// links the account to it), and deletion. On rename, references are kept valid:
/// the matching `accountLinkNode`s, the same-named `pdaNode`, and `pdaLinkNode`s
/// are all rewritten.
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
pub fn update_accounts<S: Into<String>>(
    map: impl IntoIterator<Item = (S, AccountUpdate)>,
) -> BottomUpTransformer {
    let mut rules = Vec::new();
    let mut deletes: Vec<String> = Vec::new();

    for (name, update) in map {
        let name = name.into();

        if update.delete {
            deletes.push(scoped_selector("accountNode", &name));
            continue;
        }

        let new_name = update.name.clone();
        let renames = rename_map(update.data.clone());
        // Shared across the account rule (which fills it) and the program rule
        // (which upserts it); the account is visited before its program.
        let pdas_to_upsert: Rc<RefCell<Vec<(PdaNode, String)>>> = Rc::new(RefCell::new(Vec::new()));

        rules.push(TransformRule::new(scoped_selector("accountNode", &name), {
            let update = update.clone();
            let renames = renames.clone();
            let pdas_to_upsert = Rc::clone(&pdas_to_upsert);
            move |node, path| {
                let Node::Account(mut account) = node else {
                    return node;
                };
                let program = path
                    .program()
                    .and_then(|d| d.name.as_ref())
                    .map(|n| n.to_string());
                if let Some(name) = &update.name {
                    account.name = name.clone().into();
                }
                if let Some(size) = update.size {
                    account.size = Some(size);
                }
                if let Some(docs) = &update.docs {
                    account.docs = docs.clone();
                }
                if !renames.is_empty() {
                    account.data = account
                        .data
                        .map_nested_type_node(|s: StructTypeNode| rename_struct_node(s, &renames));
                }
                apply_pda(&mut account, &update, program, &pdas_to_upsert);
                Node::Account(account)
            }
        }));

        rules.push(TransformRule::new("[programNode]", {
            let pdas_to_upsert = Rc::clone(&pdas_to_upsert);
            move |node, _path| {
                let Node::Program(mut program) = node else {
                    return node;
                };
                let to_upsert: Vec<PdaNode> = pdas_to_upsert
                    .borrow()
                    .iter()
                    .filter(|(_, p)| p.as_str() == program.name.as_ref())
                    .map(|(pda, _)| pda.clone())
                    .collect();
                for pda in to_upsert {
                    if let Some(slot) = program.pdas.iter_mut().find(|p| p.name == pda.name) {
                        *slot = pda;
                    } else {
                        program.pdas.push(pda);
                    }
                }
                Node::Program(program)
            }
        }));

        // On rename, keep every reference to the old name consistent.
        if let Some(new_name) = new_name {
            let account_link_name = new_name.clone();
            rules.push(TransformRule::new(
                scoped_selector("accountLinkNode", &name),
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
                scoped_selector("pdaNode", &name),
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
                scoped_selector("pdaLinkNode", &name),
                move |node, _path| match node {
                    Node::Link(LinkNode::Pda(link)) => Node::Link(LinkNode::Pda(PdaLinkNode {
                        name: pda_link_name.clone().into(),
                        program: link.program,
                    })),
                    other => other,
                },
            ));
        }
    }

    bottom_up_transformer(rules).also_delete(deletes)
}

/// Applies the `seeds`/`pda` part of an update, mirroring the upstream branch
/// logic, and records any PDA to upsert on the program.
fn apply_pda(
    account: &mut AccountNode,
    update: &AccountUpdate,
    program: Option<String>,
    pdas_to_upsert: &Rc<RefCell<Vec<(PdaNode, String)>>>,
) {
    let push = |pda: PdaNode| {
        if let Some(program) = program.clone() {
            pdas_to_upsert.borrow_mut().push((pda, program));
        }
    };
    match (&update.pda, update.seeds.clone()) {
        (Some(pda), Some(seeds)) => {
            push(PdaNode::new(pda.name.clone(), seeds));
            account.pda = Some(pda.clone());
        }
        (Some(pda), None) => {
            account.pda = Some(pda.clone());
        }
        (None, Some(seeds)) => {
            if let Some(existing) = account.pda.clone() {
                push(PdaNode::new(existing.name, seeds));
            } else {
                let pda_name = update
                    .name
                    .clone()
                    .unwrap_or_else(|| account.name.to_string());
                push(PdaNode::new(pda_name.clone(), seeds));
                account.pda = Some(PdaLinkNode::new(pda_name));
            }
        }
        (None, None) => {}
    }
}
