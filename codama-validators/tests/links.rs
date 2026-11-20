//! The validation items of links that point to no node. Links naming a program that is not in the tree
//! point outside of it, so they are not checked.

use codama_nodes::{
    AccountLinkNode, AccountNode, DefinedTypeLinkNode, DefinedTypeNode, InstructionAccountLinkNode,
    InstructionAccountNode, InstructionInputValueNode, InstructionLinkNode, InstructionNode, Node,
    NodeRef, PdaLinkNode, PdaNode, ProgramLinkNode, ProgramNode, ProvidedNode, PublicKeyTypeNode,
    RootNode, StructFieldTypeNode, StructTypeNode,
};
use codama_validators::{get_validation_items_visitor, LogLevel};
use codama_visitors_core::visit;

/// The message and path of each item of `node`, which must all be errors.
fn errors<'a>(node: impl Into<NodeRef<'a>>) -> Vec<(String, String)> {
    visit(&get_validation_items_visitor(), node)
        .into_iter()
        .map(|item| {
            assert_eq!(item.level, LogLevel::Error, "{item}");
            (item.message, item.path.to_string())
        })
        .collect()
}

fn error(message: &str, path: &str) -> (String, String) {
    (message.to_string(), path.to_string())
}

/// A `token` program with a `vault` PDA, a `config` defined type, a `mint` account and a `transfer`
/// instruction with an `authority` account, and an `other` program with an `escrow` PDA.
fn root(main: impl FnOnce(ProgramNode) -> ProgramNode) -> RootNode {
    let token = ProgramNode {
        version: "1.0.0".into(),
        pdas: vec![PdaNode::new("vault", vec![])],
        defined_types: vec![DefinedTypeNode::new("config", StructTypeNode::new(vec![]))],
        accounts: vec![AccountNode::new("mint", StructTypeNode::new(vec![]))],
        instructions: vec![InstructionNode {
            identifier: "transfer".into(),
            accounts: vec![InstructionAccountNode::new("authority", false, true)],
            ..InstructionNode::default()
        }],
        ..ProgramNode::new("token", "1111")
    };
    let other = ProgramNode {
        version: "1.0.0".into(),
        pdas: vec![PdaNode::new("escrow", vec![])],
        ..ProgramNode::new("other", "2222")
    };
    RootNode::new(main(token)).add_program(other)
}

/// A `token` program whose `user` account links to the given PDA.
fn root_with_account_pda(pda: PdaLinkNode) -> RootNode {
    root(|program| ProgramNode {
        accounts: vec![AccountNode {
            pda: Some(pda),
            ..AccountNode::new("user", StructTypeNode::new(vec![]))
        }],
        ..program
    })
}

/// A `token` program with a `swap` instruction providing the given node as `target`.
fn root_with_provided(node: impl Into<Node>) -> RootNode {
    root(|mut program| {
        program.instructions.push(InstructionNode {
            identifier: "swap".into(),
            accounts: vec![InstructionAccountNode::new("payer", true, true)],
            provides: vec![ProvidedNode::new("target", node)],
            ..InstructionNode::default()
        });
        program
    })
}

const ACCOUNT_PDA: &str = "[programNode]token.[accountNode]user.[pdaLinkNode]";
const PROVIDED: &str = "[programNode]token.[instructionNode]swap.[providedNode]target";

#[test]
fn reports_pda_links_to_missing_pdas() {
    let root = root_with_account_pda(PdaLinkNode::new("vualt"));
    assert_eq!(
        errors(&root),
        [error(
            "Pointing to a missing PDA named \"vualt\"",
            &format!("{ACCOUNT_PDA}vualt")
        )]
    );
}

#[test]
fn reports_pda_links_to_missing_pdas_of_programs_in_the_tree() {
    let root = root_with_account_pda(PdaLinkNode::new_from_program(
        "vault",
        ProgramLinkNode::new("other"),
    ));
    assert_eq!(
        errors(&root),
        [error(
            "Pointing to a missing PDA named \"vault\"",
            &format!("{ACCOUNT_PDA}vault")
        )]
    );
}

#[test]
fn accepts_pda_links_to_existing_pdas() {
    assert_eq!(
        errors(&root_with_account_pda(PdaLinkNode::new("vault"))),
        []
    );
    let escrow = PdaLinkNode::new_from_program("escrow", ProgramLinkNode::new("other"));
    assert_eq!(errors(&root_with_account_pda(escrow)), []);
}

#[test]
fn skips_links_to_programs_outside_the_tree() {
    // The associated token program is not in the tree, e.g. its client is generated separately.
    let external = || ProgramLinkNode::new("associatedToken");
    assert_eq!(
        errors(&root_with_account_pda(PdaLinkNode::new_from_program(
            "associatedToken",
            external()
        ))),
        []
    );
    let account_link = AccountLinkNode::new_from_program("token", external());
    let public_key = PublicKeyTypeNode {
        account_link: Some(account_link),
        ..PublicKeyTypeNode::new()
    };
    let defined_type_link = DefinedTypeLinkNode::new_from_program("config", external());
    let root = root(|mut program| {
        program.defined_types.push(DefinedTypeNode::new(
            "external",
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("account", public_key),
                StructFieldTypeNode::new("config", defined_type_link),
            ]),
        ));
        program
    });
    assert_eq!(errors(&root), []);
    let instruction_link = InstructionLinkNode::new_from_program("create", external());
    assert_eq!(errors(&root_with_provided(instruction_link.clone())), []);
    let instruction_account_link =
        InstructionAccountLinkNode::new_from_instruction("mint", instruction_link);
    assert_eq!(errors(&root_with_provided(instruction_account_link)), []);
}

#[test]
fn never_reports_program_links() {
    // A program link default value, e.g. the address of an external program.
    let root = root(|mut program| {
        *program.instructions[0].accounts[0].default_value = Some(
            InstructionInputValueNode::ProgramLink(ProgramLinkNode::new("associatedToken")),
        );
        program
    });
    assert_eq!(errors(&root), []);
}

#[test]
fn reports_account_links_to_missing_accounts_of_programs_in_the_tree() {
    let link = AccountLinkNode::new_from_program("mint", ProgramLinkNode::new("other"));
    let public_key = PublicKeyTypeNode {
        account_link: Some(link),
        ..PublicKeyTypeNode::new()
    };
    let root = root(|mut program| {
        program
            .defined_types
            .push(DefinedTypeNode::new("owner", public_key));
        program
    });
    assert_eq!(
        errors(&root),
        [error(
            "Pointing to a missing account named \"mint\"",
            "[programNode]token.[definedTypeNode]owner.[publicKeyTypeNode].[accountLinkNode]mint"
        )]
    );
}

#[test]
fn reports_instruction_links_to_missing_instructions() {
    let root = root_with_provided(InstructionLinkNode::new("tranfser"));
    assert_eq!(
        errors(&root),
        [error(
            "Pointing to a missing instruction named \"tranfser\"",
            &format!("{PROVIDED}.[instructionLinkNode]tranfser")
        )]
    );
    assert_eq!(
        errors(&root_with_provided(InstructionLinkNode::new("transfer"))),
        []
    );
}

#[test]
fn reports_instruction_account_links_to_missing_accounts() {
    // Without an instruction, the link points to an account of the surrounding instruction.
    let root = root_with_provided(InstructionAccountLinkNode::new("authority"));
    assert_eq!(
        errors(&root),
        [error(
            "Pointing to a missing instruction account named \"authority\"",
            &format!("{PROVIDED}.[instructionAccountLinkNode]authority")
        )]
    );
    assert_eq!(
        errors(&root_with_provided(InstructionAccountLinkNode::new(
            "payer"
        ))),
        []
    );
    let transfer = InstructionLinkNode::new("transfer");
    assert_eq!(
        errors(&root_with_provided(
            InstructionAccountLinkNode::new_from_instruction("authority", transfer)
        )),
        []
    );
}

#[test]
fn reports_only_the_missing_instruction_of_an_instruction_account_link() {
    let link = InstructionAccountLinkNode::new_from_instruction(
        "authority",
        InstructionLinkNode::new("mintTo"),
    );
    assert_eq!(
        errors(&root_with_provided(link)),
        [error(
            "Pointing to a missing instruction named \"mintTo\"",
            &format!(
                "{PROVIDED}.[instructionAccountLinkNode]authority.[instructionLinkNode]mintTo"
            )
        )]
    );
}

#[test]
fn reports_only_the_missing_instruction_of_another_program_in_the_tree() {
    let instruction =
        InstructionLinkNode::new_from_program("transfer", ProgramLinkNode::new("other"));
    let link = InstructionAccountLinkNode::new_from_instruction("authority", instruction);
    assert_eq!(
        errors(&root_with_provided(link)),
        [error(
            "Pointing to a missing instruction named \"transfer\"",
            &format!(
                "{PROVIDED}.[instructionAccountLinkNode]authority.[instructionLinkNode]transfer"
            )
        )]
    );
}
