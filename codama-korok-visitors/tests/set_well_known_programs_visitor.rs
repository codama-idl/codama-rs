use codama_korok_visitors::{KorokVisitable, SetWellKnownProgramsVisitor};
use codama_nodes::{
    AccountValueNode, CamelCaseString, InstructionAccountNode, InstructionArgumentNode,
    InstructionInputValueNode, InstructionNode, Node, PdaLinkNode, PdaSeedValueNode, PdaValueNode,
    ProgramLinkNode, ProgramNode, PublicKeyTypeNode, RootNode,
};
use std::collections::HashSet;

fn make_root_korok_with_node(root_node: RootNode) -> codama_koroks::RootKorok<'static> {
    let store = Box::leak(Box::new(
        codama_stores::RootStore::hydrate(proc_macro2::TokenStream::new()).unwrap(),
    ));
    codama_koroks::RootKorok {
        crates: vec![],
        node: Some(Node::Root(root_node)),
        store,
    }
}

fn make_root_with_pda_ref(program_link_name: &str) -> RootNode {
    let pda_value = PdaValueNode::new(
        PdaLinkNode::new_from_program("associatedToken", ProgramLinkNode::new(program_link_name)),
        vec![
            PdaSeedValueNode::new("owner", AccountValueNode::new("owner")),
            PdaSeedValueNode::new("tokenProgram", AccountValueNode::new("tokenProgram")),
            PdaSeedValueNode::new("mint", AccountValueNode::new("mint")),
        ],
    );

    let mut account = InstructionAccountNode::new("ata", true, false);
    account.default_value = Some(InstructionInputValueNode::Pda(pda_value));

    let instruction = InstructionNode {
        name: "createToken".into(),
        accounts: vec![account],
        ..InstructionNode::default()
    };

    let program = ProgramNode {
        name: "myProgram".into(),
        public_key: "1111111111111111111111111111111111111111111".into(),
        instructions: vec![instruction],
        ..ProgramNode::default()
    };

    RootNode::new(program)
}

fn make_root_with_program_link_ref(program_link_name: &str) -> RootNode {
    let mut account = InstructionAccountNode::new("programAccount", true, false);
    account.default_value = Some(InstructionInputValueNode::ProgramLink(
        ProgramLinkNode::new(program_link_name),
    ));

    let instruction = InstructionNode {
        name: "transfer".into(),
        accounts: vec![account],
        ..InstructionNode::default()
    };

    let program = ProgramNode {
        name: "myProgram".into(),
        public_key: "1111111111111111111111111111111111111111111".into(),
        instructions: vec![instruction],
        ..ProgramNode::default()
    };

    RootNode::new(program)
}

#[test]
fn it_injects_associated_token_program_from_pda_link() {
    let mut korok = make_root_korok_with_node(make_root_with_pda_ref("associatedToken"));
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert_eq!(root.additional_programs.len(), 1);
    assert_eq!(
        root.additional_programs[0].name,
        CamelCaseString::new("associatedToken")
    );
    assert_eq!(
        root.additional_programs[0].public_key,
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
    );
    assert_eq!(root.additional_programs[0].pdas.len(), 1);
    assert_eq!(root.additional_programs[0].pdas[0].seeds.len(), 3);
}

#[test]
fn it_injects_system_program_from_program_link() {
    let mut korok = make_root_korok_with_node(make_root_with_program_link_ref("system"));
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert_eq!(root.additional_programs.len(), 1);
    assert_eq!(
        root.additional_programs[0].name,
        CamelCaseString::new("system")
    );
    assert_eq!(
        root.additional_programs[0].public_key,
        "11111111111111111111111111111111"
    );
    assert!(root.additional_programs[0].pdas.is_empty());
}

#[test]
fn it_injects_token_program() {
    let mut korok = make_root_korok_with_node(make_root_with_program_link_ref("token"));
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert_eq!(root.additional_programs.len(), 1);
    assert_eq!(
        root.additional_programs[0].name,
        CamelCaseString::new("token")
    );
    assert_eq!(
        root.additional_programs[0].public_key,
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
    );
}

#[test]
fn it_injects_token2022_program() {
    let mut korok = make_root_korok_with_node(make_root_with_program_link_ref("token2022"));
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert_eq!(root.additional_programs.len(), 1);
    assert_eq!(
        root.additional_programs[0].name,
        CamelCaseString::new("token2022")
    );
    assert_eq!(
        root.additional_programs[0].public_key,
        "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
    );
}

#[test]
fn it_injects_memo_program() {
    let mut korok = make_root_korok_with_node(make_root_with_program_link_ref("memo"));
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert_eq!(root.additional_programs.len(), 1);
    assert_eq!(
        root.additional_programs[0].name,
        CamelCaseString::new("memo")
    );
    assert_eq!(
        root.additional_programs[0].public_key,
        "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
    );
}

#[test]
fn it_does_not_duplicate_existing_additional_program() {
    let mut root_node = make_root_with_pda_ref("associatedToken");
    root_node.additional_programs.push(ProgramNode::new(
        "associatedToken",
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
    ));

    let mut korok = make_root_korok_with_node(root_node);
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    // Should still be exactly 1 (the pre-existing one), not duplicated.
    assert_eq!(root.additional_programs.len(), 1);
}

#[test]
fn it_does_not_inject_when_main_program_matches() {
    let pda_value = PdaValueNode::new(
        PdaLinkNode::new_from_program("associatedToken", ProgramLinkNode::new("myProgram")),
        vec![],
    );

    let mut account = InstructionAccountNode::new("ata", true, false);
    account.default_value = Some(InstructionInputValueNode::Pda(pda_value));

    let instruction = InstructionNode {
        name: "createToken".into(),
        accounts: vec![account],
        ..InstructionNode::default()
    };

    let program = ProgramNode {
        name: "myProgram".into(),
        public_key: "1111111111111111111111111111111111111111111".into(),
        instructions: vec![instruction],
        ..ProgramNode::default()
    };

    let root_node = RootNode::new(program);
    let mut korok = make_root_korok_with_node(root_node);
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert!(root.additional_programs.is_empty());
}

#[test]
fn it_ignores_unknown_program_references() {
    let mut korok = make_root_korok_with_node(make_root_with_pda_ref("someCustomProgram"));
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert!(root.additional_programs.is_empty());
}

#[test]
fn it_does_nothing_when_korok_has_no_root_node() {
    let store = Box::leak(Box::new(
        codama_stores::RootStore::hydrate(proc_macro2::TokenStream::new()).unwrap(),
    ));
    let mut korok = codama_koroks::RootKorok {
        crates: vec![],
        node: None,
        store,
    };
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();
    assert!(korok.node.is_none());
}

#[test]
fn it_does_nothing_when_no_instructions_exist() {
    let program = ProgramNode {
        name: "myProgram".into(),
        public_key: "1111111111111111111111111111111111111111111".into(),
        ..ProgramNode::default()
    };
    let root_node = RootNode::new(program);
    let mut korok = make_root_korok_with_node(root_node);
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert!(root.additional_programs.is_empty());
}

#[test]
fn it_injects_multiple_well_known_programs() {
    let mut account1 = InstructionAccountNode::new("ata", true, false);
    account1.default_value = Some(InstructionInputValueNode::Pda(PdaValueNode::new(
        PdaLinkNode::new_from_program("associatedToken", ProgramLinkNode::new("associatedToken")),
        vec![],
    )));

    let mut account2 = InstructionAccountNode::new("sysProg", true, false);
    account2.default_value = Some(InstructionInputValueNode::ProgramLink(
        ProgramLinkNode::new("system"),
    ));

    let instruction = InstructionNode {
        name: "createAndTransfer".into(),
        accounts: vec![account1, account2],
        ..InstructionNode::default()
    };

    let program = ProgramNode {
        name: "myProgram".into(),
        public_key: "1111111111111111111111111111111111111111111".into(),
        instructions: vec![instruction],
        ..ProgramNode::default()
    };

    let root_node = RootNode::new(program);
    let mut korok = make_root_korok_with_node(root_node);
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert_eq!(root.additional_programs.len(), 2);

    let names: HashSet<CamelCaseString> = root
        .additional_programs
        .iter()
        .map(|p| p.name.clone())
        .collect();
    assert!(names.contains(&CamelCaseString::new("associatedToken")));
    assert!(names.contains(&CamelCaseString::new("system")));
}

#[test]
fn it_collects_references_from_argument_default_values() {
    let mut arg = InstructionArgumentNode::new("myArg", PublicKeyTypeNode::new());
    arg.default_value = Some(InstructionInputValueNode::Pda(PdaValueNode::new(
        PdaLinkNode::new_from_program("associatedToken", ProgramLinkNode::new("associatedToken")),
        vec![],
    )));

    let instruction = InstructionNode {
        name: "transfer".into(),
        arguments: vec![arg],
        ..InstructionNode::default()
    };

    let program = ProgramNode {
        name: "myProgram".into(),
        public_key: "1111111111111111111111111111111111111111111".into(),
        instructions: vec![instruction],
        ..ProgramNode::default()
    };

    let root_node = RootNode::new(program);
    let mut korok = make_root_korok_with_node(root_node);
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert_eq!(root.additional_programs.len(), 1);
    assert_eq!(
        root.additional_programs[0].name,
        CamelCaseString::new("associatedToken")
    );
}

#[test]
fn it_collects_references_from_extra_argument_default_values() {
    let mut arg = InstructionArgumentNode::new("extraArg", PublicKeyTypeNode::new());
    arg.default_value = Some(InstructionInputValueNode::ProgramLink(
        ProgramLinkNode::new("token"),
    ));

    let instruction = InstructionNode {
        name: "transfer".into(),
        extra_arguments: vec![arg],
        ..InstructionNode::default()
    };

    let program = ProgramNode {
        name: "myProgram".into(),
        public_key: "1111111111111111111111111111111111111111111".into(),
        instructions: vec![instruction],
        ..ProgramNode::default()
    };

    let root_node = RootNode::new(program);
    let mut korok = make_root_korok_with_node(root_node);
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert_eq!(root.additional_programs.len(), 1);
    assert_eq!(
        root.additional_programs[0].name,
        CamelCaseString::new("token")
    );
}

#[test]
fn it_collects_references_from_sub_instructions() {
    let mut sub_account = InstructionAccountNode::new("tokenProg", true, false);
    sub_account.default_value = Some(InstructionInputValueNode::ProgramLink(
        ProgramLinkNode::new("token2022"),
    ));

    let sub_instruction = InstructionNode {
        name: "innerTransfer".into(),
        accounts: vec![sub_account],
        ..InstructionNode::default()
    };

    let instruction = InstructionNode {
        name: "outerTransfer".into(),
        sub_instructions: vec![sub_instruction],
        ..InstructionNode::default()
    };

    let program = ProgramNode {
        name: "myProgram".into(),
        public_key: "1111111111111111111111111111111111111111111".into(),
        instructions: vec![instruction],
        ..ProgramNode::default()
    };

    let root_node = RootNode::new(program);
    let mut korok = make_root_korok_with_node(root_node);
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    assert_eq!(root.additional_programs.len(), 1);
    assert_eq!(
        root.additional_programs[0].name,
        CamelCaseString::new("token2022")
    );
}

#[test]
fn it_collects_references_from_additional_programs() {
    let mut account = InstructionAccountNode::new("prog", true, false);
    account.default_value = Some(InstructionInputValueNode::ProgramLink(
        ProgramLinkNode::new("memo"),
    ));

    let instruction = InstructionNode {
        name: "logMessage".into(),
        accounts: vec![account],
        ..InstructionNode::default()
    };

    let additional_program = ProgramNode {
        name: "helperProgram".into(),
        public_key: "2222222222222222222222222222222222222222222".into(),
        instructions: vec![instruction],
        ..ProgramNode::default()
    };

    let main_program = ProgramNode {
        name: "myProgram".into(),
        public_key: "1111111111111111111111111111111111111111111".into(),
        ..ProgramNode::default()
    };

    let mut root_node = RootNode::new(main_program);
    root_node.additional_programs.push(additional_program);

    let mut korok = make_root_korok_with_node(root_node);
    korok
        .accept(&mut SetWellKnownProgramsVisitor::new())
        .unwrap();

    let Some(Node::Root(root)) = &korok.node else {
        panic!("Expected root node");
    };
    // helperProgram was already there, memo should be injected.
    let names: HashSet<CamelCaseString> = root
        .additional_programs
        .iter()
        .map(|p| p.name.clone())
        .collect();
    assert!(names.contains(&CamelCaseString::new("memo")));
    assert!(names.contains(&CamelCaseString::new("helperProgram")));
}
