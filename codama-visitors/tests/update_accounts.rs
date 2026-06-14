use codama_nodes::{AccountNode, PdaLinkNode, PdaNode, ProgramNode, RootNode, StructTypeNode};
use codama_visitors::{update_accounts, AccountUpdate, TransformVisitor};
use pretty_assertions::assert_eq;

/// account `old` (with a same-named pda link) + a same-named pda `old`.
fn sample_root() -> RootNode {
    let mut account = AccountNode::new("old", StructTypeNode::new(vec![]));
    account.pda = Some(PdaLinkNode {
        name: "old".into(),
        program: None,
    });
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.accounts.push(account);
    program.pdas.push(PdaNode::new("old", vec![]));
    RootNode::new(program)
}

#[test]
fn updates_fields_and_rewrites_references_on_rename() {
    let root = update_accounts([("old", AccountUpdate::new().name("new").size(64))])
        .visit_root(sample_root());

    let account = &root.program.accounts[0];
    assert_eq!(account.name.as_ref(), "new");
    assert_eq!(account.size, Some(64));
    // The account's pda link was rewritten...
    assert_eq!(account.pda.as_ref().unwrap().name.as_ref(), "new");
    // ...and the same-named pda node too.
    assert_eq!(root.program.pdas[0].name.as_ref(), "new");
}

#[test]
fn field_only_update_does_not_touch_references() {
    let root = update_accounts([("old", AccountUpdate::new().size(8))]).visit_root(sample_root());
    assert_eq!(root.program.accounts[0].name.as_ref(), "old");
    assert_eq!(root.program.accounts[0].size, Some(8));
    assert_eq!(
        root.program.accounts[0].pda.as_ref().unwrap().name.as_ref(),
        "old"
    );
    assert_eq!(root.program.pdas[0].name.as_ref(), "old");
}
