use crate::{
    fold_account, fold_constant, fold_defined_type, fold_event, fold_instruction,
    fold_instruction_account, fold_instruction_argument, fold_instruction_remaining_accounts,
    fold_pda, fold_program, fold_resolver_value, fold_struct_field_type, fold_variable_pda_seed,
    TransformVisitor,
};
use codama_nodes::{
    AccountNode, ConstantNode, DefinedTypeNode, Docs, ErrorNode, EventNode, InstructionAccountNode,
    InstructionArgumentNode, InstructionNode, InstructionRemainingAccountsNode, PdaNode,
    ProgramNode, ResolverValueNode, StructFieldTypeNode, VariablePdaSeedNode,
};

/// Strips the documentation from every node that carries it.
///
/// The Rust counterpart of `@codama/visitors-core`'s `removeDocsVisitor`. Empty
/// [`Docs`] are skipped during serialization, so a cleared node simply omits its
/// `docs` field — matching the upstream behaviour of removing the key entirely
/// rather than blanking it to `[]`. Useful before hashing or rendering, where
/// docs are noise.
///
/// Only the fourteen node kinds that actually have a `docs` field are
/// overridden; every other node is rebuilt unchanged by the default fold.
#[derive(Debug, Default, Clone, Copy)]
pub struct RemoveDocs;

impl TransformVisitor for RemoveDocs {
    fn visit_account(&mut self, mut node: AccountNode) -> AccountNode {
        node.docs = Docs::default();
        fold_account(self, node)
    }
    fn visit_constant(&mut self, mut node: ConstantNode) -> ConstantNode {
        node.docs = Docs::default();
        fold_constant(self, node)
    }
    fn visit_defined_type(&mut self, mut node: DefinedTypeNode) -> DefinedTypeNode {
        node.docs = Docs::default();
        fold_defined_type(self, node)
    }
    fn visit_error(&mut self, mut node: ErrorNode) -> ErrorNode {
        // `ErrorNode` is a leaf, so there is nothing to recurse into.
        node.docs = Docs::default();
        node
    }
    fn visit_event(&mut self, mut node: EventNode) -> EventNode {
        node.docs = Docs::default();
        fold_event(self, node)
    }
    fn visit_instruction(&mut self, mut node: InstructionNode) -> InstructionNode {
        node.docs = Docs::default();
        fold_instruction(self, node)
    }
    fn visit_instruction_account(
        &mut self,
        mut node: InstructionAccountNode,
    ) -> InstructionAccountNode {
        node.docs = Docs::default();
        fold_instruction_account(self, node)
    }
    fn visit_instruction_argument(
        &mut self,
        mut node: InstructionArgumentNode,
    ) -> InstructionArgumentNode {
        node.docs = Docs::default();
        fold_instruction_argument(self, node)
    }
    fn visit_instruction_remaining_accounts(
        &mut self,
        mut node: InstructionRemainingAccountsNode,
    ) -> InstructionRemainingAccountsNode {
        node.docs = Docs::default();
        fold_instruction_remaining_accounts(self, node)
    }
    fn visit_pda(&mut self, mut node: PdaNode) -> PdaNode {
        node.docs = Docs::default();
        fold_pda(self, node)
    }
    fn visit_program(&mut self, mut node: ProgramNode) -> ProgramNode {
        node.docs = Docs::default();
        fold_program(self, node)
    }
    fn visit_resolver_value(&mut self, mut node: ResolverValueNode) -> ResolverValueNode {
        node.docs = Docs::default();
        fold_resolver_value(self, node)
    }
    fn visit_struct_field_type(&mut self, mut node: StructFieldTypeNode) -> StructFieldTypeNode {
        node.docs = Docs::default();
        fold_struct_field_type(self, node)
    }
    fn visit_variable_pda_seed(&mut self, mut node: VariablePdaSeedNode) -> VariablePdaSeedNode {
        node.docs = Docs::default();
        fold_variable_pda_seed(self, node)
    }
}
