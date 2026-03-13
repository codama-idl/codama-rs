use crate::KorokVisitor;
use codama_errors::CodamaResult;
use codama_nodes::{
    CamelCaseString, InstructionInputValueNode, Node, PdaNode, PdaValue, PdaValueNode, ProgramNode,
    PublicKeyTypeNode, RootNode, VariablePdaSeedNode,
};
use std::collections::HashSet;

struct WellKnownProgram {
    name: &'static str,
    address: &'static str,
    pdas: &'static [WellKnownPda],
}

struct WellKnownPda {
    name: &'static str,
    seeds: &'static [(&'static str, SeedType)],
}

#[derive(Clone, Copy)]
enum SeedType {
    PublicKey,
}

static WELL_KNOWN_PROGRAMS: &[WellKnownProgram] = &[
    WellKnownProgram {
        name: "associatedToken",
        address: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        pdas: &[WellKnownPda {
            name: "associatedToken",
            seeds: &[
                ("owner", SeedType::PublicKey),
                ("tokenProgram", SeedType::PublicKey),
                ("mint", SeedType::PublicKey),
            ],
        }],
    },
    WellKnownProgram {
        name: "system",
        address: "11111111111111111111111111111111",
        pdas: &[],
    },
    WellKnownProgram {
        name: "token",
        address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        pdas: &[],
    },
    WellKnownProgram {
        name: "token2022",
        address: "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
        pdas: &[],
    },
    WellKnownProgram {
        name: "memo",
        address: "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
        pdas: &[],
    },
];

/// Auto-injects well-known programs (ATA, system, token, etc.) into `additional_programs`
/// when referenced by `ProgramLinkNode`s in instruction default values but not yet present.
#[derive(Default)]
pub struct SetWellKnownProgramsVisitor;

impl SetWellKnownProgramsVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for SetWellKnownProgramsVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> CodamaResult<()> {
        let Some(Node::Root(root)) = &mut korok.node else {
            return Ok(());
        };

        // Collect names of all existing programs (main + additional).
        let existing: HashSet<CamelCaseString> = std::iter::once(root.program.name.clone())
            .chain(root.additional_programs.iter().map(|p| p.name.clone()))
            .collect();

        // Collect all ProgramLinkNode names referenced in instruction default values.
        let referenced = collect_program_link_names(root);

        // For each referenced program name, if it's not already present and is well-known, inject it.
        for name in &referenced {
            if existing.contains(name) {
                continue;
            }
            if let Some(program_node) = build_well_known_program(name) {
                root.additional_programs.push(program_node);
            }
        }

        Ok(())
    }
}

/// Walk the RootNode and collect all `ProgramLinkNode` names from instruction default values.
fn collect_program_link_names(root: &RootNode) -> HashSet<CamelCaseString> {
    let mut names = HashSet::new();
    collect_from_program(&root.program, &mut names);
    for program in &root.additional_programs {
        collect_from_program(program, &mut names);
    }
    names
}

fn collect_from_program(program: &ProgramNode, names: &mut HashSet<CamelCaseString>) {
    for instruction in &program.instructions {
        collect_from_instruction(instruction, names);
    }
}

fn collect_from_instruction(
    instruction: &codama_nodes::InstructionNode,
    names: &mut HashSet<CamelCaseString>,
) {
    for account in &instruction.accounts {
        if let Some(ref default_value) = account.default_value {
            collect_from_input_value(default_value, names);
        }
    }
    for arg in instruction
        .arguments
        .iter()
        .chain(instruction.extra_arguments.iter())
    {
        if let Some(ref default_value) = arg.default_value {
            collect_from_input_value(default_value, names);
        }
    }
    for sub in &instruction.sub_instructions {
        collect_from_instruction(sub, names);
    }
}

fn collect_from_input_value(
    value: &InstructionInputValueNode,
    names: &mut HashSet<CamelCaseString>,
) {
    if let InstructionInputValueNode::Pda(pda) = value {
        collect_from_pda_value(pda, names);
    }
    if let InstructionInputValueNode::ProgramLink(link) = value {
        names.insert(link.name.clone());
    }
}

fn collect_from_pda_value(pda: &PdaValueNode, names: &mut HashSet<CamelCaseString>) {
    if let PdaValue::Linked(ref link) = pda.pda {
        if let Some(ref program) = link.program {
            names.insert(program.name.clone());
        }
    }
}

/// Build a `ProgramNode` for a well-known program by name.
fn build_well_known_program(name: &CamelCaseString) -> Option<ProgramNode> {
    WELL_KNOWN_PROGRAMS
        .iter()
        .find(|wk| CamelCaseString::new(wk.name) == *name)
        .map(|wk| {
            let mut program = ProgramNode::new(wk.name, wk.address);
            for pda in wk.pdas {
                let seeds = pda
                    .seeds
                    .iter()
                    .map(|(seed_name, seed_type)| match seed_type {
                        SeedType::PublicKey => {
                            VariablePdaSeedNode::new(*seed_name, PublicKeyTypeNode::new()).into()
                        }
                    })
                    .collect();
                program.pdas.push(PdaNode::new(pda.name, seeds));
            }
            program
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KorokVisitor;
    use codama_nodes::{
        AccountValueNode, InstructionAccountNode, InstructionArgumentNode, InstructionNode,
        PdaLinkNode, PdaSeedValueNode, ProgramLinkNode, PublicKeyTypeNode,
    };

    fn make_root_with_pda_ref(program_link_name: &str) -> RootNode {
        let pda_value = PdaValueNode::new(
            PdaLinkNode::new_from_program(
                "associatedToken",
                ProgramLinkNode::new(program_link_name),
            ),
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

    fn make_root_korok_with_node(root_node: RootNode) -> codama_koroks::RootKorok<'static> {
        // Leak a RootStore so we can get a 'static reference for test purposes.
        let store = Box::leak(Box::new(
            codama_stores::RootStore::hydrate(proc_macro2::TokenStream::new()).unwrap(),
        ));
        codama_koroks::RootKorok {
            crates: vec![],
            node: Some(Node::Root(root_node)),
            store,
        }
    }

    // --- Tests for internal helpers ---

    #[test]
    fn injects_well_known_program_when_referenced() {
        let root = make_root_with_pda_ref("associatedToken");
        let referenced = collect_program_link_names(&root);
        assert!(referenced.contains(&CamelCaseString::new("associatedToken")));

        let existing: HashSet<CamelCaseString> =
            [CamelCaseString::new("myProgram")].into_iter().collect();

        let mut injected = vec![];
        for name in &referenced {
            if !existing.contains(name) {
                if let Some(p) = build_well_known_program(name) {
                    injected.push(p);
                }
            }
        }

        assert_eq!(injected.len(), 1);
        assert_eq!(injected[0].name, CamelCaseString::new("associatedToken"));
        assert_eq!(
            injected[0].public_key,
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        );
        assert_eq!(injected[0].pdas.len(), 1);
        assert_eq!(
            injected[0].pdas[0].name,
            CamelCaseString::new("associatedToken")
        );
        assert_eq!(injected[0].pdas[0].seeds.len(), 3);
    }

    #[test]
    fn does_not_duplicate_existing_program() {
        let mut root = make_root_with_pda_ref("associatedToken");
        // Pre-add the program.
        root.additional_programs.push(ProgramNode::new(
            "associatedToken",
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        ));

        let existing: HashSet<CamelCaseString> = std::iter::once(root.program.name.clone())
            .chain(root.additional_programs.iter().map(|p| p.name.clone()))
            .collect();

        let referenced = collect_program_link_names(&root);
        let mut injected = vec![];
        for name in &referenced {
            if !existing.contains(name) {
                if let Some(p) = build_well_known_program(name) {
                    injected.push(p);
                }
            }
        }

        assert_eq!(injected.len(), 0);
    }

    #[test]
    fn ignores_unknown_program_references() {
        let root = make_root_with_pda_ref("someCustomProgram");
        let referenced = collect_program_link_names(&root);
        assert!(referenced.contains(&CamelCaseString::new("someCustomProgram")));

        for name in &referenced {
            assert!(build_well_known_program(name).is_none());
        }
    }

    #[test]
    fn no_references_no_injection() {
        let program = ProgramNode {
            name: "myProgram".into(),
            public_key: "1111111111111111111111111111111111111111111".into(),
            ..ProgramNode::default()
        };
        let root = RootNode::new(program);
        let referenced = collect_program_link_names(&root);
        assert!(referenced.is_empty());
    }

    // --- Tests for build_well_known_program ---

    #[test]
    fn build_system_program() {
        let program = build_well_known_program(&CamelCaseString::new("system")).unwrap();
        assert_eq!(program.name, CamelCaseString::new("system"));
        assert_eq!(program.public_key, "11111111111111111111111111111111");
        assert!(program.pdas.is_empty());
    }

    #[test]
    fn build_token_program() {
        let program = build_well_known_program(&CamelCaseString::new("token")).unwrap();
        assert_eq!(program.name, CamelCaseString::new("token"));
        assert_eq!(
            program.public_key,
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        );
        assert!(program.pdas.is_empty());
    }

    #[test]
    fn build_token2022_program() {
        let program = build_well_known_program(&CamelCaseString::new("token2022")).unwrap();
        assert_eq!(program.name, CamelCaseString::new("token2022"));
        assert_eq!(
            program.public_key,
            "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        );
        assert!(program.pdas.is_empty());
    }

    #[test]
    fn build_memo_program() {
        let program = build_well_known_program(&CamelCaseString::new("memo")).unwrap();
        assert_eq!(program.name, CamelCaseString::new("memo"));
        assert_eq!(
            program.public_key,
            "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
        );
        assert!(program.pdas.is_empty());
    }

    #[test]
    fn build_associated_token_program_has_pdas() {
        let program = build_well_known_program(&CamelCaseString::new("associatedToken")).unwrap();
        assert_eq!(program.pdas.len(), 1);

        let pda = &program.pdas[0];
        assert_eq!(pda.name, CamelCaseString::new("associatedToken"));
        assert_eq!(pda.seeds.len(), 3);
    }

    #[test]
    fn build_unknown_program_returns_none() {
        assert!(build_well_known_program(&CamelCaseString::new("unknownProgram")).is_none());
    }

    // --- Tests for collect_program_link_names ---

    #[test]
    fn collects_program_link_from_direct_default_value() {
        let root = make_root_with_program_link_ref("system");
        let referenced = collect_program_link_names(&root);
        assert!(referenced.contains(&CamelCaseString::new("system")));
    }

    #[test]
    fn collects_from_argument_default_values() {
        let mut arg = InstructionArgumentNode::new("myArg", PublicKeyTypeNode::new());
        arg.default_value = Some(InstructionInputValueNode::Pda(PdaValueNode::new(
            PdaLinkNode::new_from_program(
                "associatedToken",
                ProgramLinkNode::new("associatedToken"),
            ),
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

        let root = RootNode::new(program);
        let referenced = collect_program_link_names(&root);
        assert!(referenced.contains(&CamelCaseString::new("associatedToken")));
    }

    #[test]
    fn collects_from_extra_argument_default_values() {
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

        let root = RootNode::new(program);
        let referenced = collect_program_link_names(&root);
        assert!(referenced.contains(&CamelCaseString::new("token")));
    }

    #[test]
    fn collects_from_sub_instructions() {
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

        let root = RootNode::new(program);
        let referenced = collect_program_link_names(&root);
        assert!(referenced.contains(&CamelCaseString::new("token2022")));
    }

    #[test]
    fn collects_from_additional_programs() {
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

        let mut root = RootNode::new(main_program);
        root.additional_programs.push(additional_program);

        let referenced = collect_program_link_names(&root);
        assert!(referenced.contains(&CamelCaseString::new("memo")));
    }

    #[test]
    fn collects_multiple_distinct_references() {
        let mut account1 = InstructionAccountNode::new("ata", true, false);
        account1.default_value = Some(InstructionInputValueNode::Pda(PdaValueNode::new(
            PdaLinkNode::new_from_program(
                "associatedToken",
                ProgramLinkNode::new("associatedToken"),
            ),
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

        let root = RootNode::new(program);
        let referenced = collect_program_link_names(&root);
        assert!(referenced.contains(&CamelCaseString::new("associatedToken")));
        assert!(referenced.contains(&CamelCaseString::new("system")));
        assert_eq!(referenced.len(), 2);
    }

    // --- Tests for visitor via visit_root ---

    #[test]
    fn visitor_injects_associated_token_program() {
        let mut korok = make_root_korok_with_node(make_root_with_pda_ref("associatedToken"));
        let mut visitor = SetWellKnownProgramsVisitor::new();
        visitor.visit_root(&mut korok).unwrap();

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
    fn visitor_injects_system_program_from_program_link() {
        let mut korok = make_root_korok_with_node(make_root_with_program_link_ref("system"));
        let mut visitor = SetWellKnownProgramsVisitor::new();
        visitor.visit_root(&mut korok).unwrap();

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
    }

    #[test]
    fn visitor_does_not_inject_already_existing_program() {
        let mut root_node = make_root_with_pda_ref("associatedToken");
        root_node.additional_programs.push(ProgramNode::new(
            "associatedToken",
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        ));

        let mut korok = make_root_korok_with_node(root_node);
        let mut visitor = SetWellKnownProgramsVisitor::new();
        visitor.visit_root(&mut korok).unwrap();

        let Some(Node::Root(root)) = &korok.node else {
            panic!("Expected root node");
        };
        // Should still be exactly 1 (the pre-existing one), not duplicated.
        assert_eq!(root.additional_programs.len(), 1);
    }

    #[test]
    fn visitor_does_not_inject_when_main_program_matches() {
        // If the main program itself is the referenced name, skip it.
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
        let mut visitor = SetWellKnownProgramsVisitor::new();
        visitor.visit_root(&mut korok).unwrap();

        let Some(Node::Root(root)) = &korok.node else {
            panic!("Expected root node");
        };
        assert!(root.additional_programs.is_empty());
    }

    #[test]
    fn visitor_ignores_unknown_references() {
        let mut korok = make_root_korok_with_node(make_root_with_pda_ref("someCustomProgram"));
        let mut visitor = SetWellKnownProgramsVisitor::new();
        visitor.visit_root(&mut korok).unwrap();

        let Some(Node::Root(root)) = &korok.node else {
            panic!("Expected root node");
        };
        assert!(root.additional_programs.is_empty());
    }

    #[test]
    fn visitor_noop_when_korok_has_no_root_node() {
        let store = Box::leak(Box::new(
            codama_stores::RootStore::hydrate(proc_macro2::TokenStream::new()).unwrap(),
        ));
        let mut korok = codama_koroks::RootKorok {
            crates: vec![],
            node: None,
            store,
        };
        let mut visitor = SetWellKnownProgramsVisitor::new();
        visitor.visit_root(&mut korok).unwrap();
        assert!(korok.node.is_none());
    }

    #[test]
    fn visitor_noop_when_no_instructions() {
        let program = ProgramNode {
            name: "myProgram".into(),
            public_key: "1111111111111111111111111111111111111111111".into(),
            ..ProgramNode::default()
        };
        let root_node = RootNode::new(program);
        let mut korok = make_root_korok_with_node(root_node);
        let mut visitor = SetWellKnownProgramsVisitor::new();
        visitor.visit_root(&mut korok).unwrap();

        let Some(Node::Root(root)) = &korok.node else {
            panic!("Expected root node");
        };
        assert!(root.additional_programs.is_empty());
    }

    #[test]
    fn visitor_injects_multiple_well_known_programs() {
        // Build a root with references to both associatedToken and system.
        let mut account1 = InstructionAccountNode::new("ata", true, false);
        account1.default_value = Some(InstructionInputValueNode::Pda(PdaValueNode::new(
            PdaLinkNode::new_from_program(
                "associatedToken",
                ProgramLinkNode::new("associatedToken"),
            ),
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
        let mut visitor = SetWellKnownProgramsVisitor::new();
        visitor.visit_root(&mut korok).unwrap();

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
}
