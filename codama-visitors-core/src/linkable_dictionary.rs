use crate::NodePath;
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{HasKind, Identifier, InstructionNode, NodeRef, ProgramNode};
use std::{cell::RefCell, collections::HashMap};

/// Finds the nodes that link nodes point to, from the paths of the nodes they can point to.
///
/// A link node, such as a `definedTypeLinkNode`, names the node it points to rather than holding it.
/// The dictionary records the path of every node that a link can point to, which
/// [`VisitorExt::record_linkables_on_first_visit`](crate::VisitorExt::record_linkables_on_first_visit)
/// does for a whole tree, and resolves a link from its own path:
///
/// - the program of the link is the one it names, or else the closest program on its path;
/// - the instruction of an `instructionAccountLinkNode` is the one it names, or else the closest
///   instruction on its path.
///
/// Its methods take `&self`, so visitors and their extensions can share it by reference.
///
/// ```
/// use codama_nodes::{DefinedTypeLinkNode, DefinedTypeNode, ProgramNode, RootNode, StringTypeNode};
/// use codama_nodes::{NodeRef, StructFieldTypeNode, StructTypeNode};
/// use codama_visitors_core::{visit, LinkableDictionary, NodePath, RecordLinkablesVisitor};
///
/// let root = RootNode::new(ProgramNode {
///     defined_types: vec![DefinedTypeNode::new("label", StringTypeNode::utf8())],
///     ..ProgramNode::new("token", "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
/// });
/// let linkables = LinkableDictionary::default();
/// visit(&RecordLinkablesVisitor::new(&linkables), &root);
///
/// // A link resolves from its own path, here in the `token` program.
/// let link = DefinedTypeLinkNode::new("label");
/// let link_path = NodePath::from(vec![NodeRef::from(&root.program), NodeRef::from(&link)]);
/// let linked = linkables.get_or_err(&link_path).unwrap();
/// assert_eq!(linked, NodeRef::from(&root.program.defined_types[0]));
/// ```
#[derive(Debug, Default)]
pub struct LinkableDictionary<'a> {
    programs: RefCell<HashMap<Identifier, ProgramDictionary<'a>>>,
}

#[derive(Debug)]
struct ProgramDictionary<'a> {
    program: NodePath<'a>,
    accounts: HashMap<Identifier, NodePath<'a>>,
    defined_types: HashMap<Identifier, NodePath<'a>>,
    instructions: HashMap<Identifier, InstructionDictionary<'a>>,
    pdas: HashMap<Identifier, NodePath<'a>>,
}

#[derive(Debug)]
struct InstructionDictionary<'a> {
    instruction: NodePath<'a>,
    accounts: HashMap<Identifier, NodePath<'a>>,
}

impl<'a> LinkableDictionary<'a> {
    /// Records the path of a node that links can point to: a program, an account, a defined type,
    /// a PDA, an instruction or an instruction account. Any other node is ignored, and so is a node
    /// outside of a program.
    pub fn record_path(&self, path: &NodePath<'a>) {
        let Some(node) = path.last() else {
            return;
        };
        let Some(program_path) = path.until_last::<ProgramNode>() else {
            return;
        };
        let program = path
            .find_last::<ProgramNode>()
            .expect("the path has a program");
        let mut programs = self.programs.borrow_mut();
        let program_dictionary = programs
            .entry(program.identifier.clone())
            .or_insert_with(|| ProgramDictionary {
                program: program_path,
                accounts: HashMap::new(),
                defined_types: HashMap::new(),
                instructions: HashMap::new(),
                pdas: HashMap::new(),
            });

        let record = |paths: &mut HashMap<Identifier, NodePath<'a>>, identifier: &Identifier| {
            paths.insert(identifier.clone(), path.clone());
        };
        match node {
            NodeRef::Account(account) => {
                record(&mut program_dictionary.accounts, &account.identifier)
            }
            NodeRef::DefinedType(defined_type) => record(
                &mut program_dictionary.defined_types,
                &defined_type.identifier,
            ),
            NodeRef::Pda(pda) => record(&mut program_dictionary.pdas, &pda.identifier),
            NodeRef::Instruction(_) | NodeRef::InstructionAccount(_) => {
                let Some(instruction_path) = path.until_last::<InstructionNode>() else {
                    return;
                };
                let instruction = path
                    .find_last::<InstructionNode>()
                    .expect("the path has an instruction");
                let instruction_dictionary = program_dictionary
                    .instructions
                    .entry(instruction.identifier.clone())
                    .or_insert_with(|| InstructionDictionary {
                        instruction: instruction_path,
                        accounts: HashMap::new(),
                    });
                if let NodeRef::InstructionAccount(account) = node {
                    record(&mut instruction_dictionary.accounts, &account.identifier);
                }
            }
            _ => {}
        }
    }

    /// The path of the node that the link at the end of `link_path` points to, if it is recorded.
    pub fn get_path(&self, link_path: &NodePath<'a>) -> Option<NodePath<'a>> {
        let link = link_path.last()?;
        let programs = self.programs.borrow();
        let program_identifier = match link {
            NodeRef::ProgramLink(link) => Some(&link.identifier),
            NodeRef::AccountLink(link) => link.program.as_ref().map(|program| &program.identifier),
            NodeRef::DefinedTypeLink(link) => {
                link.program.as_ref().map(|program| &program.identifier)
            }
            NodeRef::PdaLink(link) => link.program.as_ref().map(|program| &program.identifier),
            NodeRef::InstructionLink(link) => {
                link.program.as_ref().map(|program| &program.identifier)
            }
            NodeRef::InstructionAccountLink(link) => link
                .instruction
                .as_ref()
                .and_then(|instruction| instruction.program.as_ref())
                .map(|program| &program.identifier),
            _ => return None,
        };
        let program_identifier = program_identifier.or_else(|| {
            link_path
                .find_last::<ProgramNode>()
                .map(|program| &program.identifier)
        })?;
        let program = programs.get(program_identifier)?;

        let instruction = || {
            let instruction_identifier = match link {
                NodeRef::InstructionLink(link) => Some(&link.identifier),
                NodeRef::InstructionAccountLink(link) => link
                    .instruction
                    .as_ref()
                    .map(|instruction| &instruction.identifier),
                _ => None,
            }
            .or_else(|| {
                link_path
                    .find_last::<InstructionNode>()
                    .map(|instruction| &instruction.identifier)
            })?;
            program.instructions.get(instruction_identifier)
        };

        let path = match link {
            NodeRef::ProgramLink(_) => Some(&program.program),
            NodeRef::AccountLink(link) => program.accounts.get(&link.identifier),
            NodeRef::DefinedTypeLink(link) => program.defined_types.get(&link.identifier),
            NodeRef::PdaLink(link) => program.pdas.get(&link.identifier),
            NodeRef::InstructionLink(_) => {
                instruction().map(|instruction| &instruction.instruction)
            }
            NodeRef::InstructionAccountLink(link) => {
                instruction().and_then(|instruction| instruction.accounts.get(&link.identifier))
            }
            _ => None,
        };
        path.cloned()
    }

    /// The node that the link at the end of `link_path` points to, if it is recorded.
    pub fn get(&self, link_path: &NodePath<'a>) -> Option<NodeRef<'a>> {
        self.get_path(link_path)?.last()
    }

    /// Whether the node that the link at the end of `link_path` points to is recorded.
    pub fn has(&self, link_path: &NodePath<'a>) -> bool {
        self.get_path(link_path).is_some()
    }

    /// Like [`LinkableDictionary::get_path`], with an error naming the link when it points to no
    /// recorded node.
    pub fn get_path_or_err(&self, link_path: &NodePath<'a>) -> CodamaResult<NodePath<'a>> {
        self.get_path(link_path).ok_or_else(|| {
            let link = link_path.last();
            CodamaError::LinkedNodeNotFound {
                kind: link.map(|link| link.kind()).unwrap_or("None").to_string(),
                identifier: link
                    .and_then(|link| link.identifier())
                    .map(Identifier::to_string)
                    .unwrap_or_default(),
                path: link_path.to_string(),
            }
        })
    }

    /// Like [`LinkableDictionary::get`], with an error naming the link when it points to no recorded
    /// node.
    pub fn get_or_err(&self, link_path: &NodePath<'a>) -> CodamaResult<NodeRef<'a>> {
        Ok(self
            .get_path_or_err(link_path)?
            .last()
            .expect("a recorded path ends with its node"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{visit, RecordLinkablesVisitor};
    use codama_nodes::{
        AccountLinkNode, AccountNode, DefinedTypeLinkNode, DefinedTypeNode,
        InstructionAccountLinkNode, InstructionAccountNode, InstructionLinkNode, PdaLinkNode,
        PdaNode, ProgramLinkNode, RootNode, StringTypeNode, StructTypeNode,
    };

    fn program(identifier: &'static str) -> ProgramNode {
        ProgramNode {
            accounts: vec![AccountNode::new("mint", StructTypeNode::new(vec![]))],
            defined_types: vec![DefinedTypeNode::new("label", StringTypeNode::utf8())],
            pdas: vec![PdaNode::new("counter", vec![])],
            instructions: vec![InstructionNode {
                identifier: "transfer".into(),
                accounts: vec![InstructionAccountNode::new("source", true, false)],
                ..InstructionNode::default()
            }],
            ..ProgramNode::new(identifier, "1111")
        }
    }

    fn root() -> RootNode {
        RootNode {
            additional_programs: vec![program("system")],
            ..RootNode::new(program("token"))
        }
    }

    fn record(root: &RootNode) -> LinkableDictionary<'_> {
        let linkables = LinkableDictionary::default();
        visit(&RecordLinkablesVisitor::new(&linkables), root);
        linkables
    }

    /// The path of `link`, as if it were in the program `program` of `root`.
    fn link_path<'a>(program: &'a ProgramNode, link: NodeRef<'a>) -> NodePath<'a> {
        NodePath::from(vec![NodeRef::from(program), link])
    }

    fn resolved<'a>(linkables: &LinkableDictionary<'a>, path: &NodePath<'a>) -> Option<String> {
        linkables.get_path(path).map(|path| path.to_string())
    }

    #[test]
    fn resolves_each_kind_of_link_in_the_program_of_its_path() {
        let root = root();
        let linkables = record(&root);
        let token = &root.program;
        let account = AccountLinkNode::new("mint");
        let defined_type = DefinedTypeLinkNode::new("label");
        let pda = PdaLinkNode::new("counter");
        let instruction = InstructionLinkNode::new("transfer");
        let program = ProgramLinkNode::new("token");
        for (link, expected) in [
            (NodeRef::from(&account), "[accountNode]mint"),
            (NodeRef::from(&defined_type), "[definedTypeNode]label"),
            (NodeRef::from(&pda), "[pdaNode]counter"),
            (NodeRef::from(&instruction), "[instructionNode]transfer"),
        ] {
            assert_eq!(
                resolved(&linkables, &link_path(token, link)),
                Some(format!("[rootNode] > [programNode]token > {expected}"))
            );
        }
        assert_eq!(
            resolved(&linkables, &link_path(token, NodeRef::from(&program))),
            Some("[rootNode] > [programNode]token".to_string())
        );
    }

    #[test]
    fn resolves_a_link_in_the_program_it_names() {
        let root = root();
        let linkables = record(&root);
        let link = DefinedTypeLinkNode::new_from_program("label", ProgramLinkNode::new("system"));
        assert_eq!(
            resolved(&linkables, &link_path(&root.program, NodeRef::from(&link))),
            Some("[rootNode] > [programNode]system > [definedTypeNode]label".to_string())
        );
    }

    #[test]
    fn resolves_an_instruction_account_in_the_closest_or_named_instruction() {
        let root = root();
        let linkables = record(&root);
        let instruction = &root.program.instructions[0];
        let link = InstructionAccountLinkNode::new("source");
        let path = NodePath::from(vec![
            NodeRef::from(&root.program),
            NodeRef::from(instruction),
            NodeRef::from(&link),
        ]);
        let expected =
            "[rootNode] > [programNode]token > [instructionNode]transfer > [instructionAccountNode]source";
        assert_eq!(resolved(&linkables, &path), Some(expected.to_string()));

        let link = InstructionAccountLinkNode::new_from_instruction(
            "source",
            InstructionLinkNode::new("transfer"),
        );
        assert_eq!(
            resolved(&linkables, &link_path(&root.program, NodeRef::from(&link))),
            Some(expected.to_string())
        );
    }

    #[test]
    fn missing_links() {
        let root = root();
        let linkables = record(&root);
        let missing = DefinedTypeLinkNode::new("missing");
        let path = link_path(&root.program, NodeRef::from(&missing));
        assert_eq!(linkables.get(&path), None);
        assert!(!linkables.has(&path));
        assert_eq!(
            linkables.get_path_or_err(&path).unwrap_err().to_string(),
            "Could not find the node that the `definedTypeLinkNode` named `missing` links to, at [programNode]token > [definedTypeLinkNode]missing"
        );

        let other_program =
            DefinedTypeLinkNode::new_from_program("label", ProgramLinkNode::new("missing"));
        assert!(!linkables.has(&link_path(&root.program, NodeRef::from(&other_program))));

        // Outside of a program, a link that names no program cannot resolve.
        let label = DefinedTypeLinkNode::new("label");
        assert!(!linkables.has(&NodePath::from(vec![NodeRef::from(&label)])));
    }

    #[test]
    fn ignores_nodes_outside_of_a_program() {
        let defined_type = DefinedTypeNode::new("label", StringTypeNode::utf8());
        let linkables = LinkableDictionary::default();
        visit(&RecordLinkablesVisitor::new(&linkables), &defined_type);
        assert!(linkables.programs.borrow().is_empty());
    }
}
