use codama_nodes::{
    AccountNode, NestedTypeNodeTrait, NumberTypeNode, PdaLinkNode, PdaNode, PdaSeedNode,
    ProgramNode, PublicKeyTypeNode, RootNode, StructFieldTypeNode, StructTypeNode,
    VariablePdaSeedNode, U8,
};
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

#[test]
fn seeds_upsert_the_existing_pda() {
    // `old` already has a pda link to the same-named (empty) pda; providing seeds
    // upserts that pda in place.
    let seeds = vec![PdaSeedNode::Variable(VariablePdaSeedNode::new(
        "authority",
        PublicKeyTypeNode::new(),
    ))];
    let root =
        update_accounts([("old", AccountUpdate::new().seeds(seeds))]).visit_root(sample_root());

    assert_eq!(root.program.pdas.len(), 1);
    assert_eq!(root.program.pdas[0].name.as_ref(), "old");
    assert_eq!(root.program.pdas[0].seeds.len(), 1);
}

#[test]
fn seeds_without_an_existing_pda_create_one_and_link_it() {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .accounts
        .push(AccountNode::new("config", StructTypeNode::new(vec![])));
    let root = update_accounts([(
        "config",
        AccountUpdate::new().seeds(vec![PdaSeedNode::Variable(VariablePdaSeedNode::new(
            "authority",
            PublicKeyTypeNode::new(),
        ))]),
    )])
    .visit_root(RootNode::new(program));

    // A new pda named after the account was created and linked.
    assert_eq!(root.program.pdas.len(), 1);
    assert_eq!(root.program.pdas[0].name.as_ref(), "config");
    assert_eq!(
        root.program.accounts[0].pda.as_ref().unwrap().name.as_ref(),
        "config"
    );
}

#[test]
fn renames_inner_data_fields() {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.accounts.push(AccountNode::new(
        "config",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "amount",
            NumberTypeNode::le(U8),
        )]),
    ));
    let root = update_accounts([("config", AccountUpdate::new().rename("amount", "lamports"))])
        .visit_root(RootNode::new(program));

    let data: &StructTypeNode = root.program.accounts[0].data.get_nested_type_node();
    assert_eq!(data.fields[0].name.as_ref(), "lamports");
}

#[test]
fn deletes_an_account() {
    let root = update_accounts([("old", AccountUpdate::new().delete())]).visit_root(sample_root());
    assert!(root.program.accounts.is_empty());
}
