use crate::NodePath;
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{Identifier, InstructionNode, NodeRef, ProgramNode};
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
    programs: RefCell<OrderedMap<ProgramDictionary<'a>>>,
}

#[derive(Debug)]
struct ProgramDictionary<'a> {
    program: NodePath<'a>,
    accounts: OrderedMap<NodePath<'a>>,
    defined_types: OrderedMap<NodePath<'a>>,
    instructions: OrderedMap<InstructionDictionary<'a>>,
    pdas: OrderedMap<NodePath<'a>>,
}

#[derive(Debug)]
struct InstructionDictionary<'a> {
    instruction: NodePath<'a>,
    accounts: OrderedMap<NodePath<'a>>,
}

/// A map by identifier that iterates in insertion order, like a JavaScript `Map`.
/// Inserting an existing identifier replaces its value and keeps its position.
#[derive(Debug)]
struct OrderedMap<V> {
    indices: HashMap<Identifier, usize>,
    values: Vec<V>,
}

impl<V> Default for OrderedMap<V> {
    fn default() -> Self {
        Self {
            indices: HashMap::new(),
            values: Vec::new(),
        }
    }
}

impl<V> OrderedMap<V> {
    fn get(&self, identifier: &Identifier) -> Option<&V> {
        self.indices
            .get(identifier)
            .map(|&index| &self.values[index])
    }

    fn insert(&mut self, identifier: &Identifier, value: V) {
        match self.indices.get(identifier) {
            Some(&index) => self.values[index] = value,
            None => {
                self.indices.insert(identifier.clone(), self.values.len());
                self.values.push(value);
            }
        }
    }

    fn get_or_insert_with(&mut self, identifier: &Identifier, value: impl FnOnce() -> V) -> &mut V {
        let index = match self.indices.get(identifier) {
            Some(&index) => index,
            None => {
                self.indices.insert(identifier.clone(), self.values.len());
                self.values.push(value());
                self.values.len() - 1
            }
        };
        &mut self.values[index]
    }

    fn values(&self) -> impl Iterator<Item = &V> {
        self.values.iter()
    }

    fn clear(&mut self) {
        self.indices.clear();
        self.values.clear();
    }

    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
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
        let program: &ProgramNode = program_path
            .last()
            .and_then(|node| node.try_into().ok())
            .expect("until_last ends with a program");
        let mut programs = self.programs.borrow_mut();
        let program_dictionary =
            programs.get_or_insert_with(&program.identifier, || ProgramDictionary {
                program: program_path,
                accounts: OrderedMap::default(),
                defined_types: OrderedMap::default(),
                instructions: OrderedMap::default(),
                pdas: OrderedMap::default(),
            });

        match node {
            NodeRef::Account(account) => program_dictionary
                .accounts
                .insert(&account.identifier, path.clone()),
            NodeRef::DefinedType(defined_type) => program_dictionary
                .defined_types
                .insert(&defined_type.identifier, path.clone()),
            NodeRef::Pda(pda) => program_dictionary
                .pdas
                .insert(&pda.identifier, path.clone()),
            NodeRef::Instruction(_) | NodeRef::InstructionAccount(_) => {
                let Some(instruction_path) = path.until_last::<InstructionNode>() else {
                    return;
                };
                let instruction: &InstructionNode = instruction_path
                    .last()
                    .and_then(|node| node.try_into().ok())
                    .expect("until_last ends with an instruction");
                let instruction_dictionary = program_dictionary.instructions.get_or_insert_with(
                    &instruction.identifier,
                    || InstructionDictionary {
                        instruction: instruction_path,
                        accounts: OrderedMap::default(),
                    },
                );
                if let NodeRef::InstructionAccount(account) = node {
                    instruction_dictionary
                        .accounts
                        .insert(&account.identifier, path.clone());
                }
            }
            _ => {}
        }
    }

    /// Forgets every recorded node, for example before recording the nodes of another tree.
    pub fn clear(&self) {
        self.programs.borrow_mut().clear();
    }

    /// The paths of every recorded node of type `T`, such as `get_recorded_paths_of_kind::<AccountNode>()`,
    /// across all programs and in the order they were recorded.
    pub fn get_recorded_paths_of_kind<T: 'a>(&self) -> Vec<NodePath<'a>>
    where
        &'a T: TryFrom<NodeRef<'a>>,
    {
        let programs = self.programs.borrow();
        let mut paths = Vec::new();
        for program in programs.values() {
            paths.push(&program.program);
            paths.extend(program.accounts.values());
            paths.extend(program.defined_types.values());
            paths.extend(program.pdas.values());
            for instruction in program.instructions.values() {
                paths.push(&instruction.instruction);
                paths.extend(instruction.accounts.values());
            }
        }
        // Each kind of node lives in one map only, so filtering keeps the recording order of its map.
        paths
            .into_iter()
            .filter(|path| {
                path.last()
                    .is_some_and(|node| <&'a T>::try_from(node).is_ok())
            })
            .cloned()
            .collect()
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
        self.get_path(link_path)
            .ok_or_else(|| CodamaError::LinkedNodeNotFound {
                path: match link_path.is_empty() {
                    true => "none".to_string(),
                    false => link_path.to_string(),
                },
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
                Some(format!("[programNode]token.{expected}"))
            );
        }
        assert_eq!(
            resolved(&linkables, &link_path(token, NodeRef::from(&program))),
            Some("[programNode]token".to_string())
        );
    }

    #[test]
    fn resolves_a_link_in_the_program_it_names() {
        let root = root();
        let linkables = record(&root);
        let link = DefinedTypeLinkNode::new_from_program("label", ProgramLinkNode::new("system"));
        assert_eq!(
            resolved(&linkables, &link_path(&root.program, NodeRef::from(&link))),
            Some("[programNode]system.[definedTypeNode]label".to_string())
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
            "[programNode]token.[instructionNode]transfer.[instructionAccountNode]source";
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
            "Could not find the node linked from [programNode]token.[definedTypeLinkNode]missing."
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

    #[test]
    fn recorded_paths_of_kind_keep_the_recording_order_across_programs() {
        let root = root();
        let linkables = record(&root);
        let paths =
            |paths: Vec<NodePath>| paths.iter().map(ToString::to_string).collect::<Vec<_>>();
        assert_eq!(
            paths(linkables.get_recorded_paths_of_kind::<AccountNode>()),
            [
                "[programNode]token.[accountNode]mint",
                "[programNode]system.[accountNode]mint"
            ]
        );
        assert_eq!(
            paths(linkables.get_recorded_paths_of_kind::<ProgramNode>()),
            ["[programNode]token", "[programNode]system"]
        );
        assert_eq!(
            paths(linkables.get_recorded_paths_of_kind::<InstructionAccountNode>()),
            [
                "[programNode]token.[instructionNode]transfer.[instructionAccountNode]source",
                "[programNode]system.[instructionNode]transfer.[instructionAccountNode]source",
            ]
        );
    }

    #[test]
    fn recording_an_identifier_again_replaces_its_path_in_place() {
        let program = ProgramNode {
            defined_types: vec![
                DefinedTypeNode::new("a", StringTypeNode::utf8()),
                DefinedTypeNode::new("b", StringTypeNode::utf8()),
                DefinedTypeNode::new("a", StructTypeNode::new(vec![])),
            ],
            ..ProgramNode::new("token", "1111")
        };
        let linkables = LinkableDictionary::default();
        visit(&RecordLinkablesVisitor::new(&linkables), &program);
        let paths = linkables.get_recorded_paths_of_kind::<DefinedTypeNode>();
        let linked: Vec<_> = paths.iter().map(|path| path.last().unwrap()).collect();
        assert_eq!(
            linked,
            [
                NodeRef::from(&program.defined_types[2]),
                NodeRef::from(&program.defined_types[1]),
            ]
        );
    }

    #[test]
    fn clear_forgets_every_recorded_node() {
        let root = root();
        let linkables = record(&root);
        linkables.clear();
        assert!(linkables.programs.borrow().is_empty());
        assert!(linkables
            .get_recorded_paths_of_kind::<ProgramNode>()
            .is_empty());
    }
}
