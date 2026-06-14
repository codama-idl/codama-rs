use codama_nodes::{
    AccountNode, PdaLinkNode, ProgramLinkNode, ProgramNode, RootNode, StructTypeNode,
};
use codama_visitors::{update_programs, ProgramUpdate, TransformVisitor};
use pretty_assertions::assert_eq;

/// A program "oldName" whose account references the program via a nested
/// `programLinkNode` (account.pda.program).
fn sample_root() -> RootNode {
    let mut account = AccountNode::new("acc", StructTypeNode::new(vec![]));
    account.pda = Some(PdaLinkNode {
        name: "myPda".into(),
        program: Some(ProgramLinkNode {
            name: "oldName".into(),
        }),
    });
    let mut program = ProgramNode::new("oldName", "Myprogram1111111111111111111111111111111111");
    program.accounts.push(account);
    RootNode::new(program)
}

fn linked_program_name(root: &RootNode) -> String {
    root.program.accounts[0]
        .pda
        .as_ref()
        .unwrap()
        .program
        .as_ref()
        .unwrap()
        .name
        .to_string()
}

#[test]
fn renames_the_program_and_rewrites_program_links() {
    let root = update_programs([(
        "oldName",
        ProgramUpdate::new().name("newName").version("2.0.0"),
    )])
    .visit_root(sample_root());

    assert_eq!(root.program.name.as_ref(), "newName");
    assert_eq!(root.program.version, "2.0.0");
    // The nested program link was rewritten too.
    assert_eq!(linked_program_name(&root), "newName");
}

#[test]
fn field_only_update_leaves_links_untouched() {
    let root = update_programs([("oldName", ProgramUpdate::new().version("9.9.9"))])
        .visit_root(sample_root());

    assert_eq!(root.program.name.as_ref(), "oldName");
    assert_eq!(root.program.version, "9.9.9");
    assert_eq!(linked_program_name(&root), "oldName");
}
