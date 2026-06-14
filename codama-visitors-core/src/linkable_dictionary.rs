use codama_nodes::{
    AccountLinkNode, AccountNode, DefinedTypeLinkNode, DefinedTypeNode, InstructionAccountLinkNode,
    InstructionAccountNode, InstructionArgumentLinkNode, InstructionArgumentNode,
    InstructionLinkNode, InstructionNode, PdaLinkNode, PdaNode, ProgramLinkNode, ProgramNode,
    RootNode,
};
use std::collections::HashMap;

/// Resolves link nodes (`accountLinkNode`, `definedTypeLinkNode`, …) to the
/// linkable nodes they point at.
///
/// The Rust counterpart of `@codama/visitors-core`'s `LinkableDictionary`. Build
/// it once from a [`RootNode`] with [`LinkableDictionary::from_root`], then call
/// the typed `get_*` accessors, passing the *current* program/instruction names
/// (e.g. from a [`crate::NodePath`]) so unqualified links resolve within the
/// right scope. A link's own `program`/`instruction` reference, when present,
/// takes precedence over the current scope.
///
/// Because a `RootNode`'s shape is fixed, the dictionary is populated by direct
/// iteration rather than a traversal. It holds cloned snapshots of the linkable
/// nodes.
#[derive(Debug, Default, Clone)]
pub struct LinkableDictionary {
    programs: HashMap<String, ProgramDictionary>,
}

#[derive(Debug, Clone)]
struct ProgramDictionary {
    program: ProgramNode,
    accounts: HashMap<String, AccountNode>,
    defined_types: HashMap<String, DefinedTypeNode>,
    pdas: HashMap<String, PdaNode>,
    instructions: HashMap<String, InstructionDictionary>,
}

#[derive(Debug, Clone)]
struct InstructionDictionary {
    accounts: HashMap<String, InstructionAccountNode>,
    arguments: HashMap<String, InstructionArgumentNode>,
}

impl LinkableDictionary {
    /// Records every linkable node reachable from `root`.
    pub fn from_root(root: &RootNode) -> Self {
        let mut dictionary = Self::default();
        dictionary.record_program(&root.program);
        for program in &root.additional_programs {
            dictionary.record_program(program);
        }
        dictionary
    }

    fn record_program(&mut self, program: &ProgramNode) {
        let instructions = program
            .instructions
            .iter()
            .map(|instruction| {
                let dictionary = InstructionDictionary {
                    accounts: by_name(&instruction.accounts, |a| a.name.to_string()),
                    arguments: by_name(&instruction.arguments, |a| a.name.to_string()),
                };
                (instruction.name.to_string(), dictionary)
            })
            .collect();

        self.programs.insert(
            program.name.to_string(),
            ProgramDictionary {
                accounts: by_name(&program.accounts, |a| a.name.to_string()),
                defined_types: by_name(&program.defined_types, |d| d.name.to_string()),
                pdas: by_name(&program.pdas, |p| p.name.to_string()),
                instructions,
                program: program.clone(),
            },
        );
    }

    /// The program a link points at: its explicit `program` reference, else the
    /// caller's current program.
    fn program_scope<'a>(
        &self,
        explicit: Option<&'a ProgramLinkNode>,
        current: Option<&'a str>,
    ) -> Option<&'a str> {
        explicit.map(|p| p.name.as_ref()).or(current)
    }

    pub fn get_program(&self, link: &ProgramLinkNode) -> Option<&ProgramNode> {
        self.programs.get(link.name.as_ref()).map(|p| &p.program)
    }

    pub fn get_account(
        &self,
        link: &AccountLinkNode,
        current_program: Option<&str>,
    ) -> Option<&AccountNode> {
        let program = self.program_scope(link.program.as_ref(), current_program)?;
        self.programs.get(program)?.accounts.get(link.name.as_ref())
    }

    pub fn get_defined_type(
        &self,
        link: &DefinedTypeLinkNode,
        current_program: Option<&str>,
    ) -> Option<&DefinedTypeNode> {
        let program = self.program_scope(link.program.as_ref(), current_program)?;
        self.programs
            .get(program)?
            .defined_types
            .get(link.name.as_ref())
    }

    pub fn get_pda(&self, link: &PdaLinkNode, current_program: Option<&str>) -> Option<&PdaNode> {
        let program = self.program_scope(link.program.as_ref(), current_program)?;
        self.programs.get(program)?.pdas.get(link.name.as_ref())
    }

    pub fn get_instruction(
        &self,
        link: &InstructionLinkNode,
        current_program: Option<&str>,
    ) -> Option<&InstructionNode> {
        let program = self.program_scope(link.program.as_ref(), current_program)?;
        self.programs
            .get(program)?
            .program
            .instructions
            .iter()
            .find(|i| i.name.as_ref() == link.name.as_ref())
    }

    pub fn get_instruction_account(
        &self,
        link: &InstructionAccountLinkNode,
        current_program: Option<&str>,
        current_instruction: Option<&str>,
    ) -> Option<&InstructionAccountNode> {
        let (program, instruction) = self.instruction_scope(
            link.instruction.as_ref(),
            current_program,
            current_instruction,
        )?;
        self.programs
            .get(program)?
            .instructions
            .get(instruction)?
            .accounts
            .get(link.name.as_ref())
    }

    pub fn get_instruction_argument(
        &self,
        link: &InstructionArgumentLinkNode,
        current_program: Option<&str>,
        current_instruction: Option<&str>,
    ) -> Option<&InstructionArgumentNode> {
        let (program, instruction) = self.instruction_scope(
            link.instruction.as_ref(),
            current_program,
            current_instruction,
        )?;
        self.programs
            .get(program)?
            .instructions
            .get(instruction)?
            .arguments
            .get(link.name.as_ref())
    }

    /// Resolves the (program, instruction) scope for an instruction-scoped link.
    fn instruction_scope<'a>(
        &self,
        explicit: Option<&'a InstructionLinkNode>,
        current_program: Option<&'a str>,
        current_instruction: Option<&'a str>,
    ) -> Option<(&'a str, &'a str)> {
        match explicit {
            Some(instruction) => {
                let program = self.program_scope(instruction.program.as_ref(), current_program)?;
                Some((program, instruction.name.as_ref()))
            }
            None => Some((current_program?, current_instruction?)),
        }
    }
}

fn by_name<T: Clone>(items: &[T], name: impl Fn(&T) -> String) -> HashMap<String, T> {
    items
        .iter()
        .map(|item| (name(item), item.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{InstructionNode, IsSigner, NumberTypeNode, StructTypeNode, U8};

    fn sample() -> LinkableDictionary {
        let mut program =
            ProgramNode::new("myProgram", "Myprogram1111111111111111111111111111111111");
        program
            .accounts
            .push(AccountNode::new("myAccount", StructTypeNode::new(vec![])));
        program
            .defined_types
            .push(DefinedTypeNode::new("myType", NumberTypeNode::le(U8)));
        program.pdas.push(PdaNode::new("myPda", vec![]));

        let mut instruction = InstructionNode {
            name: "myIx".into(),
            ..Default::default()
        };
        instruction.accounts.push(InstructionAccountNode::new(
            "authority",
            false,
            IsSigner::False,
        ));
        instruction.arguments.push(InstructionArgumentNode::new(
            "amount",
            NumberTypeNode::le(U8),
        ));
        program.instructions.push(instruction);

        LinkableDictionary::from_root(&RootNode::new(program))
    }

    #[test]
    fn resolves_program_scoped_links() {
        let d = sample();
        let p = Some("myProgram");
        assert_eq!(
            d.get_account(
                &AccountLinkNode {
                    name: "myAccount".into(),
                    program: None
                },
                p
            )
            .map(|a| a.name.to_string()),
            Some("myAccount".to_string())
        );
        assert!(d
            .get_defined_type(
                &DefinedTypeLinkNode {
                    name: "myType".into(),
                    program: None
                },
                p
            )
            .is_some());
        assert!(d
            .get_pda(
                &PdaLinkNode {
                    name: "myPda".into(),
                    program: None
                },
                p
            )
            .is_some());
        assert!(d
            .get_program(&ProgramLinkNode {
                name: "myProgram".into()
            })
            .is_some());
    }

    #[test]
    fn resolves_instruction_scoped_links_via_current_scope() {
        let d = sample();
        let account = d.get_instruction_account(
            &InstructionAccountLinkNode {
                name: "authority".into(),
                instruction: None,
            },
            Some("myProgram"),
            Some("myIx"),
        );
        assert_eq!(
            account.map(|a| a.name.to_string()),
            Some("authority".to_string())
        );

        let argument = d.get_instruction_argument(
            &InstructionArgumentLinkNode {
                name: "amount".into(),
                instruction: None,
            },
            Some("myProgram"),
            Some("myIx"),
        );
        assert_eq!(
            argument.map(|a| a.name.to_string()),
            Some("amount".to_string())
        );
    }

    #[test]
    fn an_explicit_program_reference_overrides_the_current_scope() {
        let d = sample();
        // No current scope, but the link names its program explicitly.
        let link = AccountLinkNode {
            name: "myAccount".into(),
            program: Some(ProgramLinkNode {
                name: "myProgram".into(),
            }),
        };
        assert!(d.get_account(&link, None).is_some());
    }

    #[test]
    fn unknown_links_resolve_to_none() {
        let d = sample();
        assert!(d
            .get_account(
                &AccountLinkNode {
                    name: "ghost".into(),
                    program: None
                },
                Some("myProgram")
            )
            .is_none());
        assert!(d
            .get_account(
                &AccountLinkNode {
                    name: "myAccount".into(),
                    program: None
                },
                Some("other")
            )
            .is_none());
    }
}
