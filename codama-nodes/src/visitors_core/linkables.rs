use crate::{
    AccountLinkNode, AccountNode, DefinedTypeLinkNode, DefinedTypeNode, HasIdentifier, Identifier,
    InstructionAccountLinkNode, InstructionAccountNode, InstructionLinkNode, InstructionNode,
    NodePath, PdaLinkNode, PdaNode, ProgramLinkNode, ProgramNode, RootNode,
};
use std::collections::HashMap;

/// The nodes that link nodes point to, indexed once per root node.
/// It indexes the programs of the root and, per program, its accounts, defined types, PDAs and instructions.
/// Per instruction, it indexes the instruction accounts.
/// Sub-instructions at any depth share the instruction namespace of their program.
/// A link matches the identifier of its target exactly, with no case conversion.
/// Getters take the path of the link, as the `NodeVisitor` gives it, to find its closest program and instruction.
/// Programs or instructions with the same identifier merge, and the last node wins among equal identifiers.
#[derive(Debug, Clone)]
pub struct Linkables<'a> {
    programs: HashMap<&'a Identifier, ProgramScope<'a>>,
}

/// The linkable nodes of one program.
#[derive(Debug, Clone)]
pub struct ProgramScope<'a> {
    program: &'a ProgramNode,
    accounts: HashMap<&'a Identifier, &'a AccountNode>,
    defined_types: HashMap<&'a Identifier, &'a DefinedTypeNode>,
    pdas: HashMap<&'a Identifier, &'a PdaNode>,
    instructions: HashMap<&'a Identifier, InstructionScope<'a>>,
}

#[derive(Debug, Clone)]
struct InstructionScope<'a> {
    instruction: &'a InstructionNode,
    accounts: HashMap<&'a Identifier, &'a InstructionAccountNode>,
}

impl<'a> Linkables<'a> {
    /// Indexes the main program of the root and its `additionalPrograms`.
    pub fn new(root: &'a RootNode) -> Self {
        let mut programs = HashMap::<_, ProgramScope>::new();
        for program in std::iter::once(&root.program).chain(&root.additional_programs) {
            programs
                .entry(&program.identifier)
                .or_insert_with(|| ProgramScope::new(program))
                .record(program);
        }
        Self { programs }
    }

    /// The program that the link points to, among all programs of the root.
    pub fn program(&self, link: &ProgramLinkNode) -> Option<&'a ProgramNode> {
        Some(self.programs.get(&link.identifier)?.program)
    }

    /// The account that the link points to, in the `program` of the link or else in the closest program of `path`.
    pub fn account(&self, link: &AccountLinkNode, path: &NodePath) -> Option<&'a AccountNode> {
        let program = self.linked_program(link.program.as_ref(), closest_program(path))?;
        program.account(&link.identifier)
    }

    /// The defined type that the link points to, in the `program` of the link or else in the closest program of `path`.
    pub fn defined_type(
        &self,
        link: &DefinedTypeLinkNode,
        path: &NodePath,
    ) -> Option<&'a DefinedTypeNode> {
        let program = self.linked_program(link.program.as_ref(), closest_program(path))?;
        program.defined_type(&link.identifier)
    }

    /// The PDA that the link points to, in the `program` of the link or else in the closest program of `path`.
    pub fn pda(&self, link: &PdaLinkNode, path: &NodePath) -> Option<&'a PdaNode> {
        let program = self.linked_program(link.program.as_ref(), closest_program(path))?;
        program.pda(&link.identifier)
    }

    /// The instruction or sub-instruction that the link points to.
    /// It is in the `program` of the link, or else in the closest program of `path`.
    pub fn instruction(
        &self,
        link: &InstructionLinkNode,
        path: &NodePath,
    ) -> Option<&'a InstructionNode> {
        Some(self.instruction_scope(link, path)?.instruction)
    }

    /// The instruction account that the link points to.
    /// It is in the instruction of the `instruction` link, or else in the closest instruction of `path`.
    pub fn instruction_account(
        &self,
        link: &InstructionAccountLinkNode,
        path: &NodePath,
    ) -> Option<&'a InstructionAccountNode> {
        let scope = match &link.instruction {
            Some(instruction) => self.instruction_scope(instruction, path)?,
            None => {
                let program = self.linked_program(None, closest_program(path))?;
                program
                    .instructions
                    .get(path.find_closest_identifier("instructionNode")?)?
            }
        };
        scope.accounts.get(&link.identifier).copied()
    }

    /// The program that a link points into: the `program` of the link, else `root_program`.
    /// `root_program` is the program that holds the link, the main or an additional one, as links are relative to it.
    pub fn linked_program(
        &self,
        link_program: Option<&ProgramLinkNode>,
        root_program: Option<&Identifier>,
    ) -> Option<&ProgramScope<'a>> {
        let identifier = match link_program {
            Some(program) => &program.identifier,
            None => root_program?,
        };
        self.programs.get(identifier)
    }

    fn instruction_scope(
        &self,
        link: &InstructionLinkNode,
        path: &NodePath,
    ) -> Option<&InstructionScope<'a>> {
        let program = self.linked_program(link.program.as_ref(), closest_program(path))?;
        program.instructions.get(&link.identifier)
    }
}

impl<'a> ProgramScope<'a> {
    fn new(program: &'a ProgramNode) -> Self {
        Self {
            program,
            accounts: HashMap::new(),
            defined_types: HashMap::new(),
            pdas: HashMap::new(),
            instructions: HashMap::new(),
        }
    }

    /// The identifier of the program.
    pub fn identifier(&self) -> &'a Identifier {
        &self.program.identifier
    }

    /// The account of the program with that identifier.
    pub fn account(&self, identifier: &Identifier) -> Option<&'a AccountNode> {
        self.accounts.get(identifier).copied()
    }

    /// The defined type of the program with that identifier.
    pub fn defined_type(&self, identifier: &Identifier) -> Option<&'a DefinedTypeNode> {
        self.defined_types.get(identifier).copied()
    }

    fn pda(&self, identifier: &Identifier) -> Option<&'a PdaNode> {
        self.pdas.get(identifier).copied()
    }

    fn record(&mut self, program: &'a ProgramNode) {
        self.accounts.extend(by_identifier(&program.accounts));
        self.defined_types
            .extend(by_identifier(&program.defined_types));
        self.pdas.extend(by_identifier(&program.pdas));
        self.record_instructions(&program.instructions);
    }

    fn record_instructions(&mut self, instructions: &'a [InstructionNode]) {
        for instruction in instructions {
            self.instructions
                .entry(&instruction.identifier)
                .or_insert_with(|| InstructionScope {
                    instruction,
                    accounts: HashMap::new(),
                })
                .accounts
                .extend(by_identifier(&instruction.accounts));
            self.record_instructions(&instruction.sub_instructions);
        }
    }
}

fn closest_program<'a>(path: &NodePath<'a>) -> Option<&'a Identifier> {
    path.find_closest_identifier("programNode")
}

fn by_identifier<T: HasIdentifier>(nodes: &[T]) -> impl Iterator<Item = (&Identifier, &T)> {
    nodes.iter().map(|node| (node.identifier(), node))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::id;
    use crate::{PathSegment::Attribute, PathSegment::Item, StructTypeNode};
    use pretty_assertions::assert_eq;

    fn account(identifier: &str) -> AccountNode {
        AccountNode::new(id(identifier), StructTypeNode::new(vec![]))
    }

    fn instruction(identifier: &str, account: &str) -> InstructionNode {
        InstructionNode {
            identifier: id(identifier),
            accounts: vec![InstructionAccountNode::new(id(account), false, false)],
            ..InstructionNode::default()
        }
    }

    fn root() -> RootNode {
        let batch = InstructionNode {
            sub_instructions: vec![instruction("approve", "delegate")],
            ..instruction("batch", "payer")
        };
        RootNode::new(
            ProgramNode::new(id("app"), "1111")
                .add_account(account("config"))
                .add_instruction(batch),
        )
        .add_program(
            ProgramNode::new(id("token"), "2222")
                .add_account(account("mint"))
                .add_defined_type(DefinedTypeNode::new(
                    id("amount"),
                    StructTypeNode::new(vec![]),
                ))
                .add_pda(PdaNode::new(id("vault"), vec![])),
        )
    }

    fn path_to_program<'a>(root: &'a RootNode, identifier: &str) -> NodePath<'a> {
        if root.program.identifier.as_str() == identifier {
            return NodePath::default().join_named(Attribute("program"), &root.program);
        }
        let index = root
            .additional_programs
            .iter()
            .position(|program| program.identifier.as_str() == identifier)
            .unwrap();
        NodePath::default().join_named(
            Item("additionalPrograms", index),
            &root.additional_programs[index],
        )
    }

    #[test]
    fn program() {
        let root = root();
        let linkables = Linkables::new(&root);
        assert_eq!(
            linkables.program(&ProgramLinkNode::new(id("app"))),
            Some(&root.program)
        );
        assert_eq!(
            linkables.program(&ProgramLinkNode::new(id("token"))),
            Some(&root.additional_programs[0])
        );
        assert_eq!(linkables.program(&ProgramLinkNode::new(id("system"))), None);
    }

    #[test]
    fn closest_program() {
        let root = root();
        let linkables = Linkables::new(&root);
        let token = &root.additional_programs[0];
        let path_to_app = path_to_program(&root, "app");
        let path_to_token = path_to_program(&root, "token");
        let mint_link = AccountLinkNode::new(id("mint"));
        assert_eq!(
            linkables.account(&mint_link, &path_to_token),
            Some(&token.accounts[0])
        );
        assert_eq!(linkables.account(&mint_link, &path_to_app), None);
        assert_eq!(
            linkables.account(&AccountLinkNode::new(id("config")), &path_to_app),
            Some(&root.program.accounts[0])
        );
        assert_eq!(
            linkables.defined_type(&DefinedTypeLinkNode::new(id("amount")), &path_to_token),
            Some(&token.defined_types[0])
        );
        assert_eq!(
            linkables.pda(&PdaLinkNode::new(id("vault")), &path_to_token),
            Some(&token.pdas[0])
        );
        assert_eq!(
            linkables.account(&AccountLinkNode::new(id("config")), &NodePath::default()),
            None
        );
    }

    #[test]
    fn named_program() {
        let root = root();
        let linkables = Linkables::new(&root);
        let token = &root.additional_programs[0];
        let path_to_app = path_to_program(&root, "app");
        let program_link = |program: &str| ProgramLinkNode::new(id(program));
        assert_eq!(
            linkables.account(
                &AccountLinkNode::new_from_program(id("mint"), program_link("token")),
                &path_to_app
            ),
            Some(&token.accounts[0])
        );
        assert_eq!(
            linkables.account(
                &AccountLinkNode::new_from_program(id("mint"), program_link("app")),
                &path_to_app
            ),
            None
        );
        assert_eq!(
            linkables.defined_type(
                &DefinedTypeLinkNode::new_from_program(id("amount"), program_link("token")),
                &path_to_app
            ),
            Some(&token.defined_types[0])
        );
        assert_eq!(
            linkables.pda(
                &PdaLinkNode::new_from_program(id("vault"), program_link("token")),
                &path_to_app
            ),
            Some(&token.pdas[0])
        );
        assert_eq!(
            linkables.pda(
                &PdaLinkNode::new_from_program(id("vault"), program_link("system")),
                &path_to_app
            ),
            None
        );
        assert_eq!(
            linkables.pda(
                &PdaLinkNode::new_from_program(id("vault"), program_link("token")),
                &NodePath::default()
            ),
            Some(&token.pdas[0])
        );
    }

    #[test]
    fn linked_program() {
        let root = root();
        let linkables = Linkables::new(&root);
        let app = &root.program;
        let token = &root.additional_programs[0];
        let token_link = ProgramLinkNode::new(id("token"));
        let system_link = ProgramLinkNode::new(id("system"));
        let system = id("system");
        let linked_program = |link_program, root_program| {
            linkables
                .linked_program(link_program, root_program)
                .map(ProgramScope::identifier)
        };
        assert_eq!(
            linked_program(None, Some(&token.identifier)),
            Some(&token.identifier)
        );
        assert_eq!(
            linked_program(Some(&token_link), Some(&app.identifier)),
            Some(&token.identifier)
        );
        assert_eq!(
            linked_program(Some(&token_link), None),
            Some(&token.identifier)
        );
        assert_eq!(
            linked_program(None, Some(&app.identifier)),
            Some(&app.identifier)
        );
        assert_eq!(
            linked_program(Some(&system_link), Some(&app.identifier)),
            None
        );
        assert_eq!(linked_program(None, Some(&system)), None);
        assert_eq!(linked_program(None, None), None);
    }

    #[test]
    fn program_scope() {
        let root = root();
        let linkables = Linkables::new(&root);
        let app = &root.program;
        let token = &root.additional_programs[0];
        let app_scope = linkables
            .linked_program(None, Some(&app.identifier))
            .unwrap();
        let token_scope = linkables
            .linked_program(None, Some(&token.identifier))
            .unwrap();
        assert_eq!(app_scope.account(&id("config")), Some(&app.accounts[0]));
        assert_eq!(app_scope.account(&id("mint")), None);
        assert_eq!(token_scope.account(&id("mint")), Some(&token.accounts[0]));
        assert_eq!(
            token_scope.defined_type(&id("amount")),
            Some(&token.defined_types[0])
        );
        assert_eq!(token_scope.pda(&id("vault")), Some(&token.pdas[0]));
        assert_eq!(token_scope.pda(&id("amount")), None);
    }

    #[test]
    fn instructions() {
        let root = root();
        let linkables = Linkables::new(&root);
        let batch = &root.program.instructions[0];
        let approve = &batch.sub_instructions[0];
        let path_to_batch_ix =
            path_to_program(&root, "app").join_named(Item("instructions", 0), batch);
        let path_to_approve_ix = path_to_batch_ix.join_named(Item("subInstructions", 0), approve);
        assert_eq!(
            linkables.instruction(&InstructionLinkNode::new(id("approve")), &path_to_batch_ix),
            Some(approve)
        );
        let delegate_link = InstructionAccountLinkNode::new(id("delegate"));
        assert_eq!(
            linkables.instruction_account(&delegate_link, &path_to_approve_ix),
            Some(&approve.accounts[0])
        );
        assert_eq!(
            linkables.instruction_account(&delegate_link, &path_to_batch_ix),
            None
        );
        let payer_link = InstructionAccountLinkNode::new_from_instruction(
            id("payer"),
            InstructionLinkNode::new(id("batch")),
        );
        assert_eq!(
            linkables.instruction_account(&payer_link, &path_to_approve_ix),
            Some(&batch.accounts[0])
        );
        assert_eq!(
            linkables.instruction_account(&payer_link, &path_to_program(&root, "token")),
            None
        );
    }

    #[test]
    fn duplicate_identifiers() {
        let root =
            RootNode::new(ProgramNode::new(id("token"), "1111").add_account(account("mint")))
                .add_program(
                    ProgramNode::new(id("token"), "2222")
                        .add_account(AccountNode {
                            pda: Some(PdaLinkNode::new(id("mint"))),
                            ..account("mint")
                        })
                        .add_account(account("vault")),
                );
        let linkables = Linkables::new(&root);
        let second = &root.additional_programs[0];
        let path_to_token = path_to_program(&root, "token");
        assert_eq!(
            linkables.program(&ProgramLinkNode::new(id("token"))),
            Some(&root.program)
        );
        assert_eq!(
            linkables.account(&AccountLinkNode::new(id("mint")), &path_to_token),
            Some(&second.accounts[0])
        );
        assert_eq!(
            linkables.account(&AccountLinkNode::new(id("vault")), &path_to_token),
            Some(&second.accounts[1])
        );
    }
}
