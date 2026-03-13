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
