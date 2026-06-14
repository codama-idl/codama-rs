use codama_nodes::{
    AccountValueNode, ArgumentValueNode, InstructionInputValueNode, InstructionNode, PdaNode,
    PdaSeedNode, PdaSeedValueNode, PdaSeedValueValue, PdaValueNode, PdaValuePda, TypeNode,
    VariablePdaSeedNode,
};
use codama_visitors_core::{fold_pda_value, LinkableDictionary, TransformVisitor};
use std::collections::HashSet;

/// Returned by [`fill_default_pda_seed_values`] in strict mode when, after
/// filling, a `pdaValueNode`'s seeds are still invalid or incomplete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidPdaSeedValues {
    pub instruction_name: String,
    pub pda_name: String,
}

impl std::fmt::Display for InvalidPdaSeedValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid PDA seed values for pda `{}` in instruction `{}`",
            self.pda_name, self.instruction_name
        )
    }
}

impl std::error::Error for InvalidPdaSeedValues {}

/// Fills in default values for variable PDA seeds that were not explicitly
/// provided, across an instruction-input value tree.
///
/// For every `pdaValueNode` reachable from `value`, each *variable* seed of the
/// resolved PDA that isn't already present gets a default: a public-key seed
/// whose name matches an instruction account becomes an `accountValueNode`; any
/// other seed whose name matches an instruction argument becomes an
/// `argumentValueNode`. Seeds that match neither are skipped.
///
/// The PDA is taken inline when `value` carries a `pdaNode`, otherwise resolved
/// through `linkables` (using `current_program` as the scope for unqualified
/// links). An unresolvable PDA leaves its `pdaValueNode` untouched.
///
/// In `strict` mode, an [`InvalidPdaSeedValues`] error is returned if any PDA
/// ends up with the wrong number of variable seeds or with a seed referencing an
/// account/argument that the instruction does not declare.
///
/// The Rust counterpart of `@codama/visitors`' `fillDefaultPdaSeedValuesVisitor`.
///
/// ```
/// use codama_nodes::{
///     InstructionAccountNode, InstructionArgumentNode, InstructionInputValueNode, InstructionNode,
///     IsSigner, NumberTypeNode, PdaNode, PdaSeedNode, PdaValueNode, PublicKeyTypeNode,
///     VariablePdaSeedNode, U8,
/// };
/// use codama_visitors::fill_default_pda_seed_values;
/// use codama_visitors_core::LinkableDictionary;
///
/// let pda = PdaNode::new(
///     "myPda",
///     vec![
///         PdaSeedNode::Variable(VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new())),
///         PdaSeedNode::Variable(VariablePdaSeedNode::new("amount", NumberTypeNode::le(U8))),
///     ],
/// );
/// let mut ix = InstructionNode { name: "transfer".into(), ..Default::default() };
/// ix.accounts.push(InstructionAccountNode::new("authority", false, IsSigner::True));
/// ix.arguments.push(InstructionArgumentNode::new("amount", NumberTypeNode::le(U8)));
///
/// let value = InstructionInputValueNode::PdaValue(PdaValueNode::new(pda, vec![]));
/// let filled = fill_default_pda_seed_values(value, &ix, &LinkableDictionary::default(), None, false).unwrap();
/// let InstructionInputValueNode::PdaValue(pda_value) = filled else { panic!() };
/// assert_eq!(pda_value.seeds.len(), 2);
/// ```
pub fn fill_default_pda_seed_values(
    value: InstructionInputValueNode,
    instruction: &InstructionNode,
    linkables: &LinkableDictionary,
    current_program: Option<&str>,
    strict: bool,
) -> Result<InstructionInputValueNode, InvalidPdaSeedValues> {
    let mut visitor = FillPdaSeeds {
        instruction,
        linkables,
        current_program,
        strict,
        error: None,
    };
    let result = visitor.visit_instruction_input_value_node(value);
    match visitor.error {
        Some(error) => Err(error),
        None => Ok(result),
    }
}

struct FillPdaSeeds<'a> {
    instruction: &'a InstructionNode,
    linkables: &'a LinkableDictionary,
    current_program: Option<&'a str>,
    strict: bool,
    error: Option<InvalidPdaSeedValues>,
}

impl TransformVisitor for FillPdaSeeds<'_> {
    fn visit_pda_value(&mut self, node: PdaValueNode) -> PdaValueNode {
        let node = fold_pda_value(self, node);

        // Resolve the PDA: inline when present, otherwise via the dictionary.
        let pda: Option<PdaNode> = match &*node.pda {
            PdaValuePda::Pda(pda) => Some(pda.clone()),
            PdaValuePda::PdaLink(link) => {
                self.linkables.get_pda(link, self.current_program).cloned()
            }
        };
        let Some(pda) = pda else {
            return node;
        };

        let seeds = self.fill_missing_seeds(&pda, node.seeds);

        if self.strict && self.error.is_none() && !self.all_seeds_are_valid(&pda, &seeds) {
            self.error = Some(InvalidPdaSeedValues {
                instruction_name: self.instruction.name.to_string(),
                pda_name: pda.name.to_string(),
            });
        }

        PdaValueNode { seeds, ..node }
    }
}

impl FillPdaSeeds<'_> {
    fn fill_missing_seeds(
        &self,
        pda: &PdaNode,
        existing: Vec<PdaSeedValueNode>,
    ) -> Vec<PdaSeedValueNode> {
        let existing_names: HashSet<String> = existing.iter().map(|s| s.name.to_string()).collect();
        let mut seeds = existing;
        for seed in &pda.seeds {
            let PdaSeedNode::Variable(variable) = seed else {
                continue;
            };
            if existing_names.contains(variable.name.as_ref()) {
                continue;
            }
            if let Some(default) = self.default_seed_value(variable) {
                seeds.push(default);
            }
        }
        seeds
    }

    fn default_seed_value(&self, variable: &VariablePdaSeedNode) -> Option<PdaSeedValueNode> {
        let name = variable.name.as_ref();
        let is_public_key = matches!(&*variable.r#type, TypeNode::PublicKey(_));
        if is_public_key && self.has_account(name) {
            return Some(PdaSeedValueNode::new(name, AccountValueNode::new(name)));
        }
        if self.has_argument(name) {
            return Some(PdaSeedValueNode::new(name, ArgumentValueNode::new(name)));
        }
        None
    }

    fn has_account(&self, name: &str) -> bool {
        self.instruction
            .accounts
            .iter()
            .any(|a| a.name.as_ref() == name)
    }

    fn has_argument(&self, name: &str) -> bool {
        self.instruction
            .arguments
            .iter()
            .chain(self.instruction.extra_arguments.iter())
            .any(|a| a.name.as_ref() == name)
    }

    fn all_seeds_are_valid(&self, pda: &PdaNode, seeds: &[PdaSeedValueNode]) -> bool {
        let variable_count = pda
            .seeds
            .iter()
            .filter(|s| matches!(s, PdaSeedNode::Variable(_)))
            .count();
        if variable_count != seeds.len() {
            return false;
        }
        seeds.iter().all(|seed| match &*seed.value {
            PdaSeedValueValue::Account(account) => self.has_account(account.name.as_ref()),
            PdaSeedValueValue::Argument(argument) => self.has_argument(argument.name.as_ref()),
            _ => true,
        })
    }
}
