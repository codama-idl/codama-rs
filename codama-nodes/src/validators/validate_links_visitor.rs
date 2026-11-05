use crate::{
    walk_account_link, walk_defined_type_link, walk_instruction_account_link,
    walk_instruction_link, walk_pda_link, walk_program_link, AccountLinkNode, DefinedTypeLinkNode,
    Identifier, InstructionAccountLinkNode, InstructionLinkNode, Linkables, NodePath, NodeVisitor,
    PdaLinkNode, ProgramLinkNode, RootNode,
};

/// Node visitor that lists the links of a root node that point to no node in `errors`, one message per link.
/// `Linkables` gives the resolution rules of each link kind.
/// When the `program` or `instruction` link inside a link points to nothing, only that inner link is reported.
pub struct ValidateLinksVisitor<'r> {
    linkables: Linkables<'r>,
    pub errors: Vec<String>,
}

impl<'r> ValidateLinksVisitor<'r> {
    pub fn new(root: &'r RootNode) -> Self {
        Self {
            linkables: Linkables::new(root),
            errors: Vec::new(),
        }
    }

    fn report_missing(&mut self, target: &str, identifier: &Identifier, path: &NodePath) {
        self.errors.push(format!(
            "Pointing to a missing {target} named \"{identifier}\" in {path}."
        ));
    }

    fn has_program(&self, program: Option<&ProgramLinkNode>) -> bool {
        program.is_none_or(|program| self.linkables.program(program).is_some())
    }

    fn has_instruction(&self, instruction: Option<&InstructionLinkNode>, path: &NodePath) -> bool {
        instruction
            .is_none_or(|instruction| self.linkables.instruction(instruction, path).is_some())
    }
}

impl NodeVisitor for ValidateLinksVisitor<'_> {
    fn visit_account_link<'a>(&mut self, node: &'a AccountLinkNode, path: &NodePath<'a>) {
        if self.has_program(node.program.as_ref()) && self.linkables.account(node, path).is_none() {
            self.report_missing("account", &node.identifier, path);
        }
        walk_account_link(self, node, path);
    }

    fn visit_defined_type_link<'a>(&mut self, node: &'a DefinedTypeLinkNode, path: &NodePath<'a>) {
        if self.has_program(node.program.as_ref())
            && self.linkables.defined_type(node, path).is_none()
        {
            self.report_missing("defined type", &node.identifier, path);
        }
        walk_defined_type_link(self, node, path);
    }

    fn visit_instruction_link<'a>(&mut self, node: &'a InstructionLinkNode, path: &NodePath<'a>) {
        if self.has_program(node.program.as_ref())
            && self.linkables.instruction(node, path).is_none()
        {
            self.report_missing("instruction", &node.identifier, path);
        }
        walk_instruction_link(self, node, path);
    }

    fn visit_instruction_account_link<'a>(
        &mut self,
        node: &'a InstructionAccountLinkNode,
        path: &NodePath<'a>,
    ) {
        if self.has_instruction(node.instruction.as_ref(), path)
            && self.linkables.instruction_account(node, path).is_none()
        {
            self.report_missing("instruction account", &node.identifier, path);
        }
        walk_instruction_account_link(self, node, path);
    }

    fn visit_pda_link<'a>(&mut self, node: &'a PdaLinkNode, path: &NodePath<'a>) {
        if self.has_program(node.program.as_ref()) && self.linkables.pda(node, path).is_none() {
            self.report_missing("PDA", &node.identifier, path);
        }
        walk_pda_link(self, node, path);
    }

    fn visit_program_link<'a>(&mut self, node: &'a ProgramLinkNode, path: &NodePath<'a>) {
        if self.linkables.program(node).is_none() {
            self.report_missing("program", &node.identifier, path);
        }
        walk_program_link(self, node, path);
    }
}
